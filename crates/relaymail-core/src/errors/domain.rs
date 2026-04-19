use crate::config::ConfigError;
use crate::idempotency::IdempotencyError;
use crate::message_source::MessageSourceError;
use crate::object_store::ObjectStoreError;

/// Top-level domain error aggregating the specific sub-errors.
///
/// Applications convert this to `anyhow::Error` at their outer boundary.
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("idempotency error: {0}")]
    Idempotency(#[from] IdempotencyError),

    #[error("object store error: {0}")]
    ObjectStore(#[from] ObjectStoreError),

    #[error("message source error: {0}")]
    MessageSource(#[from] MessageSourceError),
}
