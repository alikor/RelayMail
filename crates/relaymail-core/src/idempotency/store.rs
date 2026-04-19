use async_trait::async_trait;
use chrono::{DateTime, Utc};

use super::error::IdempotencyError;
use super::key::IdempotencyKey;

/// Status recorded for an in-flight or finished idempotency claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaimStatus {
    Processing,
    Sent,
    Failed,
}

/// Outcome returned from [`IdempotencyStore::claim`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClaimOutcome {
    Claimed,
    AlreadyClaimed { status: ClaimStatus },
}

/// Metadata recorded with a new claim; backends may persist any subset.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaimMetadata {
    pub claimed_at: DateTime<Utc>,
    pub ttl_seconds: u64,
}

/// Storage used to de-duplicate at-least-once deliveries.
#[async_trait]
pub trait IdempotencyStore: Send + Sync + std::fmt::Debug {
    async fn claim(
        &self,
        key: &IdempotencyKey,
        metadata: ClaimMetadata,
    ) -> Result<ClaimOutcome, IdempotencyError>;

    async fn mark_sent(
        &self,
        key: &IdempotencyKey,
        provider_message_id: &str,
    ) -> Result<(), IdempotencyError>;

    async fn mark_failed(&self, key: &IdempotencyKey, reason: &str)
        -> Result<(), IdempotencyError>;
}
