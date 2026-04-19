use async_trait::async_trait;
use relaymail_core::idempotency::{
    ClaimMetadata, ClaimOutcome, IdempotencyError, IdempotencyKey, IdempotencyStore,
    InMemoryIdempotencyStore,
};

/// Fake idempotency store with a recorded call log for assertions.
#[derive(Debug, Default)]
pub struct FakeIdempotencyStore {
    inner: InMemoryIdempotencyStore,
}

impl FakeIdempotencyStore {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl IdempotencyStore for FakeIdempotencyStore {
    async fn claim(
        &self,
        key: &IdempotencyKey,
        metadata: ClaimMetadata,
    ) -> Result<ClaimOutcome, IdempotencyError> {
        self.inner.claim(key, metadata).await
    }

    async fn mark_sent(&self, key: &IdempotencyKey, id: &str) -> Result<(), IdempotencyError> {
        self.inner.mark_sent(key, id).await
    }

    async fn mark_failed(
        &self,
        key: &IdempotencyKey,
        reason: &str,
    ) -> Result<(), IdempotencyError> {
        self.inner.mark_failed(key, reason).await
    }
}
