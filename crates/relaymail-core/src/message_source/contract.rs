use async_trait::async_trait;

use super::envelope::RawEnvelope;
use super::error::MessageSourceError;

/// Pull-based message source (queue, subscription, etc).
#[async_trait]
pub trait MessageSource: Send + Sync + std::fmt::Debug {
    /// Pull up to the adapter-configured batch size. Blocks up to the
    /// adapter's poll window; may return an empty vec on timeout.
    async fn receive(&self) -> Result<Vec<RawEnvelope>, MessageSourceError>;

    /// Acknowledge completion — permanently remove the message.
    async fn ack(&self, envelope: &RawEnvelope) -> Result<(), MessageSourceError>;

    /// Negative acknowledge — return the message for later redelivery.
    async fn nack(&self, envelope: &RawEnvelope) -> Result<(), MessageSourceError>;

    /// Extend the processing lease by the given number of seconds.
    async fn extend_visibility(
        &self,
        envelope: &RawEnvelope,
        seconds: u32,
    ) -> Result<(), MessageSourceError>;
}
