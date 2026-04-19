use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use relaymail_core::idempotency::{IdempotencyError, IdempotencyKey};

use super::store::DynamoIdempotencyStore;

pub(crate) async fn update_status(
    store: &DynamoIdempotencyStore,
    key: &IdempotencyKey,
    status: &str,
    detail: Option<&str>,
) -> Result<(), IdempotencyError> {
    let cfg = store.config();
    let mut names = HashMap::new();
    names.insert("#s".to_string(), "status".to_string());
    let mut values = HashMap::new();
    values.insert(":s".to_string(), AttributeValue::S(status.into()));
    let mut expr = "SET #s = :s".to_string();
    if let Some(d) = detail {
        names.insert("#d".into(), "detail".into());
        values.insert(":d".into(), AttributeValue::S(d.to_string()));
        expr.push_str(", #d = :d");
    }
    store
        .client()
        .update_item()
        .table_name(&cfg.table_name)
        .key(
            &cfg.pk_attribute,
            AttributeValue::S(key.as_str().to_string()),
        )
        .update_expression(expr)
        .set_expression_attribute_names(Some(names))
        .set_expression_attribute_values(Some(values))
        .send()
        .await
        .map_err(|e| IdempotencyError::Permanent(format!("{e:?}")))?;
    Ok(())
}
