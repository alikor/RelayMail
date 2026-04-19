use async_trait::async_trait;

use super::capabilities::ProviderCapabilities;
use super::error::SendError;
use super::request::SendRequest;
use super::result::SendResult;

/// Capability trait implemented by every outbound-delivery provider.
///
/// Implementations live in adapter crates (e.g. `relaymail-aws::ses`) so
/// the core domain stays provider-agnostic.
#[async_trait]
pub trait EmailSender: Send + Sync + std::fmt::Debug {
    fn capabilities(&self) -> ProviderCapabilities;

    async fn send(&self, request: SendRequest) -> Result<SendResult, SendError>;
}
