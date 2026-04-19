/// Errors raised by idempotency stores.
#[derive(Debug, thiserror::Error)]
pub enum IdempotencyError {
    #[error("backend transient failure: {0}")]
    Transient(String),

    #[error("backend permanent failure: {0}")]
    Permanent(String),

    #[error("record for key {0} not found")]
    NotFound(String),
}
