use std::sync::Arc;

use relaymail_aws::ddb::{DynamoIdempotencyStore, DynamoIdempotencyStoreConfig};
use relaymail_aws::sqs::SqsConsumerConfig;
use relaymail_aws::{S3EventParser, S3ObjectStore, SqsConsumer};
use relaymail_core::idempotency::{IdempotencyStore, InMemoryIdempotencyStore};
use relaymail_core::{Clock, DispositionPolicy, SystemClock, TenantId};
use relaymail_delivery::EmailSender;
use relaymail_runtime::pipeline::{EventParser, PipelineCtx, ProcessingConfig};

use super::aws::AwsClients;
use crate::config::AppConfig;

pub(crate) fn build_pipeline(
    cfg: &AppConfig,
    clients: &AwsClients,
    email_sender: Arc<dyn EmailSender>,
) -> PipelineCtx {
    let s3_store = Arc::new(S3ObjectStore::new(clients.s3.clone()));
    let sqs_consumer = Arc::new(SqsConsumer::new(
        clients.sqs.clone(),
        SqsConsumerConfig {
            queue_url: cfg.sqs.queue_url.clone(),
            max_messages: cfg.sqs.max_messages,
            wait_time_seconds: cfg.sqs.wait_time_seconds,
            visibility_timeout_seconds: cfg.sqs.visibility_timeout_seconds,
        },
    ));
    let idempotency_store: Arc<dyn IdempotencyStore> = match &cfg.runtime.idempotency_table_name {
        Some(table) => Arc::new(DynamoIdempotencyStore::new(
            clients.dynamo.clone(),
            DynamoIdempotencyStoreConfig::new(table),
        )),
        None => Arc::new(InMemoryIdempotencyStore::new()),
    };
    let event_parser: Arc<dyn EventParser> = Arc::new(S3EventParser::new());
    let clock: Arc<dyn Clock> = Arc::new(SystemClock);
    let tenant_id = cfg
        .general
        .tenant_id
        .as_deref()
        .and_then(|v| TenantId::parse(v).ok());
    PipelineCtx {
        object_store: s3_store,
        message_source: sqs_consumer,
        idempotency_store,
        email_sender,
        event_parser,
        clock,
        disposition_policy: DispositionPolicy::default(),
        config: Arc::new(ProcessingConfig {
            service_name: cfg.general.service_name.clone(),
            provider_label: "relay".into(),
            bucket_allowlist: cfg.s3_filter.bucket_allowlist.clone(),
            prefix_allowlist: cfg.s3_filter.prefix_allowlist.clone(),
            supported_extensions: cfg.s3_filter.supported_extensions.clone(),
            max_object_size_bytes: cfg.s3_filter.max_email_bytes,
            success_mode: cfg.processing.success_mode,
            failure_mode: cfg.processing.failure_mode,
            success_prefix: cfg.processing.success_prefix.clone(),
            failure_prefix: cfg.processing.failure_prefix.clone(),
            delete_unsupported_messages: cfg.processing.delete_unsupported_messages,
            delete_invalid_email_messages: cfg.processing.delete_invalid_email_messages,
            dry_run: cfg.general.dry_run,
            idempotency_ttl_seconds: cfg.runtime.idempotency_ttl_seconds,
        }),
        tenant_id,
    }
}
