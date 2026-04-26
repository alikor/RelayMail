use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use relaymail_runtime::{
    normalize_email, EmailEventRecord, EventRecordStatus, MessageLogRecord, SendAttemptRecord,
    SuppressionRecord, TransportStore, TransportStoreError,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct DynamoTransportStore {
    client: Client,
    table_name: String,
}

impl DynamoTransportStore {
    pub fn new(client: Client, table_name: impl Into<String>) -> Self {
        Self {
            client,
            table_name: table_name.into(),
        }
    }
}

#[async_trait]
impl TransportStore for DynamoTransportStore {
    async fn is_suppressed(
        &self,
        email_address: &str,
        stream: &str,
    ) -> Result<bool, TransportStoreError> {
        let email = normalize_email(email_address);
        for stream_key in [stream.to_ascii_lowercase(), "*".into()] {
            let out = self
                .client
                .get_item()
                .table_name(&self.table_name)
                .key("pk", av(format!("SUPPRESSION#{email}#{stream_key}")))
                .key("sk", av("ACTIVE"))
                .send()
                .await
                .map_err(|e| TransportStoreError::Unavailable(format!("{e:?}")))?;
            if out.item.is_some() {
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn record_send_attempt(
        &self,
        record: SendAttemptRecord,
    ) -> Result<(), TransportStoreError> {
        let mut item = HashMap::from([
            (
                "pk".into(),
                av(format!("MESSAGE#{}", record.internal_message_id)),
            ),
            (
                "sk".into(),
                av(format!(
                    "ATTEMPT#{:06}#{}",
                    record.attempt_number, record.provider
                )),
            ),
            ("provider".into(), av(record.provider)),
            ("attempt_number".into(), avn(record.attempt_number)),
            (
                "started_at_utc".into(),
                av(record.started_at_utc.to_rfc3339()),
            ),
            (
                "completed_at_utc".into(),
                av(record.completed_at_utc.to_rfc3339()),
            ),
            ("status".into(), av(record.status)),
        ]);
        insert_opt(&mut item, "provider_message_id", record.provider_message_id);
        insert_opt(&mut item, "error_code", record.error_code);
        insert_opt(&mut item, "error_message", record.error_message);
        self.client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .send()
            .await
            .map_err(|e| TransportStoreError::Unavailable(format!("{e:?}")))?;
        Ok(())
    }

    async fn record_message(&self, record: MessageLogRecord) -> Result<(), TransportStoreError> {
        let mut item = HashMap::from([
            (
                "pk".into(),
                av(format!("MESSAGE#{}", record.internal_message_id)),
            ),
            ("sk".into(), av("LOG")),
            ("stream".into(), av(record.stream)),
            ("provider".into(), av(record.provider)),
            ("status".into(), av(record.status)),
            ("attempt_count".into(), avn(record.attempt_count)),
            (
                "created_at_utc".into(),
                av(record.created_at_utc.to_rfc3339()),
            ),
        ]);
        insert_opt(&mut item, "correlation_id", record.correlation_id);
        insert_opt(&mut item, "provider_message_id", record.provider_message_id);
        if let Some(ts) = record.accepted_at_utc {
            item.insert("accepted_at_utc".into(), av(ts.to_rfc3339()));
        }
        insert_opt(&mut item, "error_message", record.error_message);
        self.client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .send()
            .await
            .map_err(|e| TransportStoreError::Unavailable(format!("{e:?}")))?;
        Ok(())
    }

    async fn record_event(
        &self,
        record: EmailEventRecord,
    ) -> Result<EventRecordStatus, TransportStoreError> {
        let mut item = HashMap::from([
            (
                "pk".into(),
                av(format!("EVENT#{}", record.deduplication_key)),
            ),
            ("sk".into(), av("EVENT")),
            ("provider".into(), av(record.provider)),
            ("event_type".into(), av(format!("{:?}", record.event_type))),
            (
                "received_at_utc".into(),
                av(record.received_at_utc.to_rfc3339()),
            ),
        ]);
        insert_opt(&mut item, "provider_event_id", record.provider_event_id);
        insert_opt(&mut item, "provider_message_id", record.provider_message_id);
        insert_opt(&mut item, "internal_message_id", record.internal_message_id);
        insert_opt(&mut item, "recipient", record.recipient);
        insert_opt(&mut item, "stream", record.stream);
        if let Some(ts) = record.occurred_at_utc {
            item.insert("occurred_at_utc".into(), av(ts.to_rfc3339()));
        }
        insert_opt(&mut item, "raw_payload", record.raw_payload);
        let result = self
            .client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .condition_expression("attribute_not_exists(pk)")
            .send()
            .await;
        match result {
            Ok(_) => Ok(EventRecordStatus::Accepted),
            Err(e) if format!("{e:?}").contains("ConditionalCheckFailed") => {
                Ok(EventRecordStatus::Duplicate)
            }
            Err(e) => Err(TransportStoreError::Unavailable(format!("{e:?}"))),
        }
    }

    async fn suppress(&self, record: SuppressionRecord) -> Result<(), TransportStoreError> {
        let stream = record
            .stream
            .unwrap_or_else(|| "*".into())
            .to_ascii_lowercase();
        let mut item = HashMap::from([
            (
                "pk".into(),
                av(format!(
                    "SUPPRESSION#{}#{}",
                    record.email_address_normalized, stream
                )),
            ),
            ("sk".into(), av("ACTIVE")),
            ("reason".into(), av(record.reason)),
            (
                "created_at_utc".into(),
                av(record.created_at_utc.to_rfc3339()),
            ),
        ]);
        insert_opt(&mut item, "source_provider", record.source_provider);
        insert_opt(&mut item, "source_event_id", record.source_event_id);
        if let Some(ts) = record.expires_at_utc {
            item.insert("expires_at_utc".into(), av(ts.to_rfc3339()));
        }
        insert_opt(&mut item, "notes", record.notes);
        self.client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .send()
            .await
            .map_err(|e| TransportStoreError::Unavailable(format!("{e:?}")))?;
        Ok(())
    }
}

fn av(value: impl Into<String>) -> AttributeValue {
    AttributeValue::S(value.into())
}

fn avn(value: u32) -> AttributeValue {
    AttributeValue::N(value.to_string())
}

fn insert_opt(item: &mut HashMap<String, AttributeValue>, key: &str, value: Option<String>) {
    if let Some(value) = value.filter(|v| !v.is_empty()) {
        item.insert(key.into(), av(value));
    }
}
