use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use aws_sdk_dynamodb::types::AttributeValue;
use relaymail_core::idempotency::{
    ClaimMetadata, ClaimOutcome, ClaimStatus, IdempotencyError, IdempotencyKey,
};

use super::error_map::{map_put_item_err, PutItemOutcome};
use super::store::DynamoIdempotencyStore;

pub(crate) async fn put_claim(
    store: &DynamoIdempotencyStore,
    key: &IdempotencyKey,
    metadata: ClaimMetadata,
) -> Result<ClaimOutcome, IdempotencyError> {
    let cfg = store.config();
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert(
        cfg.pk_attribute.clone(),
        AttributeValue::S(key.as_str().to_string()),
    );
    item.insert("status".into(), AttributeValue::S("processing".into()));
    item.insert(
        "created_at".into(),
        AttributeValue::S(metadata.claimed_at.to_rfc3339()),
    );
    let ttl_epoch = now_epoch() + metadata.ttl_seconds as i64;
    item.insert(
        cfg.ttl_attribute.clone(),
        AttributeValue::N(ttl_epoch.to_string()),
    );
    let result = store
        .client()
        .put_item()
        .table_name(&cfg.table_name)
        .set_item(Some(item))
        .condition_expression(format!("attribute_not_exists({})", cfg.pk_attribute))
        .send()
        .await;
    match result {
        Ok(_) => Ok(ClaimOutcome::Claimed),
        Err(e) => match map_put_item_err(e) {
            PutItemOutcome::AlreadyClaimed => Ok(ClaimOutcome::AlreadyClaimed {
                status: ClaimStatus::Processing,
            }),
            PutItemOutcome::Err(err) => Err(err),
        },
    }
}

fn now_epoch() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
