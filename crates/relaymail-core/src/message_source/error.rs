/// Errors raised by message-source adapters.
#[derive(Debug, thiserror::Error)]
pub enum MessageSourceError {
    #[error("transient transport error: {0}")]
    Transient(String),

    #[error("permanent transport error: {0}")]
    Permanent(String),

    #[error("invalid envelope payload: {0}")]
    InvalidEnvelope(String),
}
