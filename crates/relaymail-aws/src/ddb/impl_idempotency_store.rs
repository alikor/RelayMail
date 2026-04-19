use async_trait::async_trait;
use relaymail_core::idempotency::{
    ClaimMetadata, ClaimOutcome, IdempotencyError, IdempotencyKey, IdempotencyStore,
};

use super::claim::put_claim;
use super::store::DynamoIdempotencyStore;
use super::update::update_status;

#[async_trait]
impl IdempotencyStore for DynamoIdempotencyStore {
    async fn claim(
        &self,
        key: &IdempotencyKey,
        metadata: ClaimMetadata,
    ) -> Result<ClaimOutcome, IdempotencyError> {
        put_claim(self, key, metadata).await
    }

    async fn mark_sent(
        &self,
        key: &IdempotencyKey,
        provider_message_id: &str,
    ) -> Result<(), IdempotencyError> {
        update_status(self, key, "sent", Some(provider_message_id)).await
    }

    async fn mark_failed(
        &self,
        key: &IdempotencyKey,
        reason: &str,
    ) -> Result<(), IdempotencyError> {
        update_status(self, key, "failed", Some(reason)).await
    }
}
