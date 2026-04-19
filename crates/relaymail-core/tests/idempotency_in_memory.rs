use chrono::Utc;
use relaymail_core::idempotency::{
    ClaimMetadata, ClaimOutcome, ClaimStatus, IdempotencyStore, InMemoryIdempotencyStore,
};
use relaymail_core::{IdempotencyKey, ObjectId, TenantId};

fn sample_key() -> IdempotencyKey {
    let t = TenantId::parse("acme").unwrap();
    let o = ObjectId::new("bucket", "incoming/a.eml");
    IdempotencyKey::compute(Some(&t), &o, "etag-v1", 128)
}

fn metadata() -> ClaimMetadata {
    ClaimMetadata {
        claimed_at: Utc::now(),
        ttl_seconds: 3600,
    }
}

#[tokio::test]
async fn first_claim_succeeds_second_returns_status() {
    let store = InMemoryIdempotencyStore::new();
    let key = sample_key();
    let first = store.claim(&key, metadata()).await.unwrap();
    assert_eq!(first, ClaimOutcome::Claimed);
    let second = store.claim(&key, metadata()).await.unwrap();
    assert_eq!(
        second,
        ClaimOutcome::AlreadyClaimed {
            status: ClaimStatus::Processing
        }
    );
}

#[tokio::test]
async fn mark_sent_transitions_status() {
    let store = InMemoryIdempotencyStore::new();
    let key = sample_key();
    store.claim(&key, metadata()).await.unwrap();
    store.mark_sent(&key, "ses-0001").await.unwrap();
    let again = store.claim(&key, metadata()).await.unwrap();
    assert_eq!(
        again,
        ClaimOutcome::AlreadyClaimed {
            status: ClaimStatus::Sent
        }
    );
}

#[tokio::test]
async fn mark_failed_transitions_status() {
    let store = InMemoryIdempotencyStore::new();
    let key = sample_key();
    store.claim(&key, metadata()).await.unwrap();
    store.mark_failed(&key, "oops").await.unwrap();
    let again = store.claim(&key, metadata()).await.unwrap();
    assert_eq!(
        again,
        ClaimOutcome::AlreadyClaimed {
            status: ClaimStatus::Failed
        }
    );
}

#[tokio::test]
async fn mark_without_claim_is_not_found() {
    let store = InMemoryIdempotencyStore::new();
    let key = sample_key();
    assert!(store.mark_sent(&key, "x").await.is_err());
}
