use std::sync::Arc;

use relaymail_core::idempotency::IdempotencyStore;
use relaymail_core::message_source::MessageSource;
use relaymail_core::object_store::ObjectStore;
use relaymail_core::{Clock, DispositionPolicy, TenantId};
use relaymail_delivery::EmailSender;

use super::config::ProcessingConfig;
use super::event_parser::EventParser;

/// Bundle of dependencies injected into the pipeline.
#[derive(Clone)]
pub struct PipelineCtx {
    pub object_store: Arc<dyn ObjectStore>,
    pub message_source: Arc<dyn MessageSource>,
    pub idempotency_store: Arc<dyn IdempotencyStore>,
    pub email_sender: Arc<dyn EmailSender>,
    pub event_parser: Arc<dyn EventParser>,
    pub clock: Arc<dyn Clock>,
    pub disposition_policy: DispositionPolicy,
    pub config: Arc<ProcessingConfig>,
    pub tenant_id: Option<TenantId>,
}

impl std::fmt::Debug for PipelineCtx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipelineCtx")
            .field("config", &self.config)
            .field("tenant_id", &self.tenant_id)
            .field("disposition_policy", &self.disposition_policy)
            .finish_non_exhaustive()
    }
}
