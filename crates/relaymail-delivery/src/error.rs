use relaymail_core::ErrorClassification;

/// Delivery failure classes. Adapters map provider-specific errors onto
/// these variants, and [`SendError::classify`] maps them onto the shared
/// [`ErrorClassification`] used by the disposition policy.
#[derive(Debug, thiserror::Error)]
pub enum SendError {
    #[error("provider throttled the send: {0}")]
    Throttled(String),

    #[error("provider quota exceeded: {0}")]
    QuotaExceeded(String),

    #[error("provider rejected the message as invalid: {0}")]
    Validation(String),

    #[error("authentication failed: {0}")]
    AuthenticationFailure(String),

    #[error("invalid recipient: {0}")]
    InvalidRecipient(String),

    #[error("transient provider failure: {0}")]
    Transient(String),

    #[error("permanent provider failure: {0}")]
    Permanent(String),
}

impl SendError {
    pub fn classify(&self) -> ErrorClassification {
        match self {
            Self::Throttled(_) | Self::Transient(_) => ErrorClassification::Transient,
            Self::QuotaExceeded(_) | Self::Permanent(_) => ErrorClassification::PermanentSender,
            Self::AuthenticationFailure(_) => ErrorClassification::PermanentSender,
            Self::InvalidRecipient(_) => ErrorClassification::PermanentRecipient,
            Self::Validation(_) => ErrorClassification::Validation,
        }
    }
}
