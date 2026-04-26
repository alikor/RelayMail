use std::collections::BTreeMap;
use std::time::Duration;

use relaymail_runtime::{DeliveryPolicy, StreamPolicy, WebhookAuthConfig, WebhookConfig};

use super::flat::FlatConfig;

#[derive(Clone, Debug)]
pub(crate) struct DeliveryConfig {
    pub policy: DeliveryPolicy,
    pub default_provider_chain: Vec<String>,
    pub provider_timeout: Duration,
    pub resend_api_key: Option<String>,
    pub resend_base_url: String,
    pub postmark_server_token: Option<String>,
    pub postmark_base_url: String,
    pub postmark_message_streams: BTreeMap<String, String>,
    pub smtp2go_api_key: Option<String>,
    pub smtp2go_base_url: String,
    pub aws_ses_enabled: bool,
    pub webhook: WebhookConfig,
}

impl DeliveryConfig {
    pub(crate) fn from_flat(flat: &FlatConfig) -> Self {
        let fallback_enabled = flat.fallback_enabled.unwrap_or(true);
        let primary = flat
            .primary_provider
            .clone()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "resend".into());
        let mut default_provider_chain = vec![primary];
        if fallback_enabled {
            default_provider_chain.extend(csv(flat
                .fallback_providers
                .as_deref()
                .unwrap_or("postmark,smtp2go")));
        }
        if flat.aws_ses_enabled.unwrap_or(false)
            && !default_provider_chain
                .iter()
                .any(|name| name.eq_ignore_ascii_case("ses"))
        {
            default_provider_chain.push("ses".into());
        }
        let mut streams = BTreeMap::new();
        for name in csv(flat.streams.as_deref().unwrap_or("transactional,marketing")) {
            let lower = name.to_ascii_lowercase();
            let policy = match lower.as_str() {
                "marketing" => stream_policy(
                    &lower,
                    &flat.stream_marketing_allowed_from_domains,
                    &flat.stream_marketing_from_default,
                    &flat.stream_marketing_reply_to_default,
                    flat.stream_marketing_require_unsubscribe.unwrap_or(true),
                    flat.stream_marketing_require_consent_metadata
                        .unwrap_or(true),
                    &flat.stream_marketing_provider_chain,
                ),
                _ => stream_policy(
                    &lower,
                    &flat.stream_transactional_allowed_from_domains,
                    &flat.stream_transactional_from_default,
                    &flat.stream_transactional_reply_to_default,
                    flat.stream_transactional_require_unsubscribe
                        .unwrap_or(false),
                    flat.stream_transactional_require_consent_metadata
                        .unwrap_or(false),
                    &flat.stream_transactional_provider_chain,
                ),
            };
            streams.insert(lower, policy);
        }
        Self {
            policy: DeliveryPolicy {
                streams,
                default_stream: "transactional".into(),
                fallback_enabled,
                global_max_per_minute: flat.global_max_per_minute.unwrap_or(60),
            },
            default_provider_chain,
            provider_timeout: Duration::from_secs(flat.provider_timeout_seconds.unwrap_or(10)),
            resend_api_key: env_any(&["RESEND_API_KEY", "RELAYMAIL_RESEND_API_KEY"]),
            resend_base_url: flat
                .resend_base_url
                .clone()
                .unwrap_or_else(|| "https://api.resend.com".into()),
            postmark_server_token: env_any(&[
                "POSTMARK_SERVER_TOKEN",
                "RELAYMAIL_POSTMARK_SERVER_TOKEN",
            ]),
            postmark_base_url: flat
                .postmark_base_url
                .clone()
                .unwrap_or_else(|| "https://api.postmarkapp.com".into()),
            postmark_message_streams: postmark_streams(flat),
            smtp2go_api_key: env_any(&["SMTP2GO_API_KEY", "RELAYMAIL_SMTP2GO_API_KEY"]),
            smtp2go_base_url: flat
                .smtp2go_base_url
                .clone()
                .unwrap_or_else(|| "https://api.smtp2go.com/v3".into()),
            aws_ses_enabled: flat.aws_ses_enabled.unwrap_or(false),
            webhook: WebhookConfig {
                auth: WebhookAuthConfig {
                    resend_secret: env_any(&[
                        "RESEND_WEBHOOK_SECRET",
                        "RELAYMAIL_RESEND_WEBHOOK_SECRET",
                    ]),
                    postmark_username: env_any(&[
                        "POSTMARK_WEBHOOK_USERNAME",
                        "RELAYMAIL_POSTMARK_WEBHOOK_USERNAME",
                    ]),
                    postmark_password: env_any(&[
                        "POSTMARK_WEBHOOK_PASSWORD",
                        "RELAYMAIL_POSTMARK_WEBHOOK_PASSWORD",
                    ]),
                    smtp2go_auth_token: env_any(&[
                        "SMTP2GO_WEBHOOK_AUTH_TOKEN",
                        "RELAYMAIL_SMTP2GO_WEBHOOK_AUTH_TOKEN",
                    ]),
                },
                store_raw_payloads: flat.webhook_store_raw_payloads.unwrap_or(false),
            },
        }
    }
}

fn stream_policy(
    name: &str,
    domains: &Option<String>,
    default_from: &Option<String>,
    default_reply_to: &Option<String>,
    require_unsubscribe: bool,
    require_consent_metadata: bool,
    provider_chain: &Option<String>,
) -> StreamPolicy {
    StreamPolicy {
        name: name.into(),
        allowed_from_domains: csv(domains.as_deref().unwrap_or_default()),
        default_from: default_from.clone().filter(|s| !s.is_empty()),
        default_reply_to: default_reply_to.clone().filter(|s| !s.is_empty()),
        require_unsubscribe,
        require_consent_metadata,
        provider_chain: csv(provider_chain.as_deref().unwrap_or_default()),
    }
}

fn postmark_streams(flat: &FlatConfig) -> BTreeMap<String, String> {
    let default = flat
        .postmark_message_stream
        .clone()
        .unwrap_or_else(|| "outbound".into());
    BTreeMap::from([
        (
            "transactional".into(),
            flat.postmark_transactional_message_stream
                .clone()
                .unwrap_or_else(|| default.clone()),
        ),
        (
            "marketing".into(),
            flat.postmark_marketing_message_stream
                .clone()
                .unwrap_or(default),
        ),
    ])
}

fn csv(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_ascii_lowercase())
        .collect()
}

fn env_any(names: &[&str]) -> Option<String> {
    names
        .iter()
        .find_map(|name| std::env::var(name).ok().filter(|v| !v.is_empty()))
}
