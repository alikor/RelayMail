use std::collections::BTreeMap;
use std::sync::Arc;

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use chrono::Utc;
use relaymail_aws::ses::SesRuntimeConfig;
use relaymail_aws::DynamoTransportStore;
use relaymail_aws::SesSender;
use relaymail_delivery::{EmailSender, ProviderCapabilities, SendError, SendRequest, SendResult};
use relaymail_providers::{
    PostmarkConfig, PostmarkSender, ResendConfig, ResendSender, Smtp2GoConfig, Smtp2GoSender,
};
use relaymail_runtime::{InMemoryTransportStore, RelayMailDeliveryService, TransportStore};

use super::aws::AwsClients;
use crate::config::AppConfig;

pub(crate) struct DeliveryWiring {
    pub sender: Arc<dyn EmailSender>,
    pub store: Arc<dyn TransportStore>,
}

pub(crate) fn build_delivery(
    cfg: &AppConfig,
    clients: &AwsClients,
) -> anyhow::Result<DeliveryWiring> {
    let store: Arc<dyn TransportStore> = match &cfg.runtime.transport_state_table_name {
        Some(table) => Arc::new(DynamoTransportStore::new(clients.dynamo.clone(), table)),
        None => {
            tracing::warn!(
                target: "relaymail_email_ses",
                "RELAYMAIL_TRANSPORT_STATE_TABLE_NAME unset: using in-memory transport state (UNSAFE for prod)"
            );
            Arc::new(InMemoryTransportStore::new())
        }
    };
    let provider_map = provider_map(cfg, clients)?;
    let default_chain = match resolve_chain(&cfg.delivery.default_provider_chain, &provider_map) {
        Ok(chain) => chain,
        Err(err) if cfg.general.dry_run => {
            tracing::warn!(
                target: "relaymail_email_ses",
                error = %err,
                "no provider credentials configured; dry-run mode will use a noop sender"
            );
            vec![Arc::new(NoopSender) as Arc<dyn EmailSender>]
        }
        Err(err) => return Err(err).context("resolving default provider chain"),
    };
    let mut stream_chains = BTreeMap::new();
    for (stream, policy) in &cfg.delivery.policy.streams {
        if !policy.provider_chain.is_empty() {
            stream_chains.insert(
                stream.clone(),
                resolve_chain(&policy.provider_chain, &provider_map)
                    .with_context(|| format!("resolving provider chain for stream `{stream}`"))?,
            );
        }
    }
    let sender: Arc<dyn EmailSender> = Arc::new(RelayMailDeliveryService::new(
        cfg.delivery.policy.clone(),
        default_chain,
        stream_chains,
        store.clone(),
    ));
    Ok(DeliveryWiring { sender, store })
}

#[derive(Debug)]
struct NoopSender;

#[async_trait]
impl EmailSender for NoopSender {
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            provider_label: "dry-run",
            max_message_bytes: u64::MAX,
            supports_raw_mime: true,
            supports_custom_headers: true,
        }
    }

    async fn send(&self, _request: SendRequest) -> Result<SendResult, SendError> {
        Ok(SendResult::new("dry-run", Utc::now()).with_metadata("provider", "dry-run"))
    }
}

fn provider_map(
    cfg: &AppConfig,
    clients: &AwsClients,
) -> anyhow::Result<BTreeMap<String, Arc<dyn EmailSender>>> {
    let mut providers: BTreeMap<String, Arc<dyn EmailSender>> = BTreeMap::new();
    if let Some(api_key) = &cfg.delivery.resend_api_key {
        providers.insert(
            "resend".into(),
            Arc::new(ResendSender::new(ResendConfig {
                api_key: api_key.clone(),
                base_url: cfg.delivery.resend_base_url.clone(),
                timeout: cfg.delivery.provider_timeout,
            })),
        );
    }
    if let Some(server_token) = &cfg.delivery.postmark_server_token {
        providers.insert(
            "postmark".into(),
            Arc::new(PostmarkSender::new(PostmarkConfig {
                server_token: server_token.clone(),
                base_url: cfg.delivery.postmark_base_url.clone(),
                timeout: cfg.delivery.provider_timeout,
                message_streams: cfg.delivery.postmark_message_streams.clone(),
            })),
        );
    }
    if let Some(api_key) = &cfg.delivery.smtp2go_api_key {
        providers.insert(
            "smtp2go".into(),
            Arc::new(Smtp2GoSender::new(Smtp2GoConfig {
                api_key: api_key.clone(),
                base_url: cfg.delivery.smtp2go_base_url.clone(),
                timeout: cfg.delivery.provider_timeout,
            })),
        );
    }
    if cfg.delivery.aws_ses_enabled {
        providers.insert(
            "ses".into(),
            Arc::new(SesSender::new(
                clients.ses.clone(),
                SesRuntimeConfig {
                    configuration_set: cfg.ses.configuration_set.clone(),
                    source_arn: cfg.ses.source_arn.clone(),
                    from_arn: cfg.ses.from_arn.clone(),
                    return_path_arn: cfg.ses.return_path_arn.clone(),
                },
            )),
        );
    }
    Ok(providers)
}

fn resolve_chain(
    names: &[String],
    providers: &BTreeMap<String, Arc<dyn EmailSender>>,
) -> anyhow::Result<Vec<Arc<dyn EmailSender>>> {
    let mut chain = Vec::new();
    let mut missing = Vec::new();
    for name in names {
        let lower = name.to_ascii_lowercase();
        match providers.get(&lower) {
            Some(provider) => chain.push(provider.clone()),
            None => missing.push(lower),
        }
    }
    if chain.is_empty() {
        return Err(anyhow!(
            "no configured providers are available; missing credentials or disabled providers: {}",
            missing.join(", ")
        ));
    }
    Ok(chain)
}
