use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use super::error::IdempotencyError;
use super::key::IdempotencyKey;
use super::store::{ClaimMetadata, ClaimOutcome, ClaimStatus, IdempotencyStore};

/// Process-local idempotency store.
///
/// WARNING: not safe across restarts or multiple replicas. Use the DynamoDB
/// adapter in production. Intended for local dev and unit tests only.
///
/// TTL is a production concern that lives in DynamoDB; this implementation
/// keeps every entry until the process dies.
#[derive(Clone, Debug, Default)]
pub struct InMemoryIdempotencyStore {
    inner: Arc<Mutex<HashMap<String, ClaimStatus>>>,
}

impl InMemoryIdempotencyStore {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl IdempotencyStore for InMemoryIdempotencyStore {
    async fn claim(
        &self,
        key: &IdempotencyKey,
        _metadata: ClaimMetadata,
    ) -> Result<ClaimOutcome, IdempotencyError> {
        let mut map = self.inner.lock().await;
        if let Some(status) = map.get(key.as_str()) {
            return Ok(ClaimOutcome::AlreadyClaimed { status: *status });
        }
        map.insert(key.as_str().to_string(), ClaimStatus::Processing);
        Ok(ClaimOutcome::Claimed)
    }

    async fn mark_sent(
        &self,
        key: &IdempotencyKey,
        _provider_message_id: &str,
    ) -> Result<(), IdempotencyError> {
        let mut map = self.inner.lock().await;
        match map.get_mut(key.as_str()) {
            Some(s) => {
                *s = ClaimStatus::Sent;
                Ok(())
            }
            None => Err(IdempotencyError::NotFound(key.as_str().to_string())),
        }
    }

    async fn mark_failed(
        &self,
        key: &IdempotencyKey,
        _reason: &str,
    ) -> Result<(), IdempotencyError> {
        let mut map = self.inner.lock().await;
        match map.get_mut(key.as_str()) {
            Some(s) => {
                *s = ClaimStatus::Failed;
                Ok(())
            }
            None => Err(IdempotencyError::NotFound(key.as_str().to_string())),
        }
    }
}
