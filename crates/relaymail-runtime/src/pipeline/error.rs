use relaymail_core::idempotency::IdempotencyError;
use relaymail_core::message_source::MessageSourceError;
use relaymail_core::object_store::ObjectStoreError;
use relaymail_core::ErrorClassification;
use relaymail_delivery::SendError;
use relaymail_email::EmailError;

use super::event_parser::EventParseError;

/// Union of errors any pipeline stage may produce.
#[derive(Debug, thiserror::Error)]
pub enum StageError {
    #[error("event parse: {0}")]
    EventParse(#[from] EventParseError),

    #[error("object fetch: {0}")]
    Fetch(#[from] ObjectStoreError),

    #[error("email validation: {0}")]
    Validate(#[from] EmailError),

    #[error("idempotency: {0}")]
    Claim(#[from] IdempotencyError),

    #[error("send: {0}")]
    Send(#[from] SendError),

    #[error("message source: {0}")]
    Source(#[from] MessageSourceError),
}

impl StageError {
    pub fn classification(&self) -> ErrorClassification {
        match self {
            Self::EventParse(_) => ErrorClassification::Validation,
            Self::Validate(_) => ErrorClassification::Validation,
            Self::Send(e) => e.classify(),
            Self::Fetch(ObjectStoreError::NotFound(_)) => ErrorClassification::Validation,
            Self::Fetch(ObjectStoreError::TooLarge { .. }) => ErrorClassification::Validation,
            Self::Fetch(ObjectStoreError::PermissionDenied(_)) => {
                ErrorClassification::PermanentSender
            }
            Self::Fetch(ObjectStoreError::Transient(_)) => ErrorClassification::Transient,
            Self::Fetch(ObjectStoreError::Permanent(_)) => ErrorClassification::PermanentSender,
            Self::Claim(IdempotencyError::Transient(_)) => ErrorClassification::Transient,
            Self::Claim(_) => ErrorClassification::PermanentSender,
            Self::Source(MessageSourceError::Transient(_)) => ErrorClassification::Transient,
            Self::Source(_) => ErrorClassification::PermanentSender,
        }
    }
}

#[cfg(test)]
#[path = "error_tests.rs"]
mod tests;
