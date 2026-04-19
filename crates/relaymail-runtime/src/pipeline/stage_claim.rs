use chrono::{DateTime, Utc};
use relaymail_core::idempotency::{ClaimMetadata, ClaimOutcome, ClaimStatus, IdempotencyStore};
use relaymail_core::{IdempotencyKey, TenantId};

use super::error::StageError;
use super::event_parser::ObjectRef;

pub(crate) fn compute_key(tenant: Option<&TenantId>, object: &ObjectRef) -> IdempotencyKey {
    IdempotencyKey::compute(tenant, &object.object, &object.etag, object.size)
}

pub(crate) async fn claim(
    store: &dyn IdempotencyStore,
    key: &IdempotencyKey,
    at: DateTime<Utc>,
    ttl_seconds: u64,
) -> Result<ClaimDecision, StageError> {
    let outcome = store
        .claim(
            key,
            ClaimMetadata {
                claimed_at: at,
                ttl_seconds,
            },
        )
        .await?;
    Ok(map(outcome))
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ClaimDecision {
    Proceed,
    AlreadyClaimed,
    AlreadySent,
}

fn map(outcome: ClaimOutcome) -> ClaimDecision {
    match outcome {
        ClaimOutcome::Claimed => ClaimDecision::Proceed,
        ClaimOutcome::AlreadyClaimed {
            status: ClaimStatus::Sent,
        } => ClaimDecision::AlreadySent,
        ClaimOutcome::AlreadyClaimed { .. } => ClaimDecision::AlreadyClaimed,
    }
}
