use async_trait::async_trait;
use bytes::Bytes;
use relaymail_core::message_source::{MessageSource, MessageSourceError};
use relaymail_core::RawEnvelope;

use super::consumer::SqsConsumer;
use super::error_map::map_sdk_error;

#[async_trait]
impl MessageSource for SqsConsumer {
    async fn receive(&self) -> Result<Vec<RawEnvelope>, MessageSourceError> {
        let cfg = self.config();
        let resp = self
            .client()
            .receive_message()
            .queue_url(&cfg.queue_url)
            .max_number_of_messages(cfg.max_messages)
            .wait_time_seconds(cfg.wait_time_seconds)
            .visibility_timeout(cfg.visibility_timeout_seconds)
            .send()
            .await
            .map_err(|e| map_sdk_error(e, "receive_message"))?;
        Ok(resp
            .messages
            .unwrap_or_default()
            .into_iter()
            .map(|m| {
                RawEnvelope::new(
                    m.message_id.unwrap_or_default(),
                    Bytes::from(m.body.unwrap_or_default()),
                    m.receipt_handle.unwrap_or_default(),
                )
            })
            .collect())
    }

    async fn ack(&self, envelope: &RawEnvelope) -> Result<(), MessageSourceError> {
        self.client()
            .delete_message()
            .queue_url(&self.config().queue_url)
            .receipt_handle(envelope.receipt_handle())
            .send()
            .await
            .map_err(|e| map_sdk_error(e, "delete_message"))?;
        Ok(())
    }

    async fn nack(&self, envelope: &RawEnvelope) -> Result<(), MessageSourceError> {
        self.extend_visibility(envelope, 0).await
    }

    async fn extend_visibility(
        &self,
        envelope: &RawEnvelope,
        seconds: u32,
    ) -> Result<(), MessageSourceError> {
        self.client()
            .change_message_visibility()
            .queue_url(&self.config().queue_url)
            .receipt_handle(envelope.receipt_handle())
            .visibility_timeout(seconds as i32)
            .send()
            .await
            .map_err(|e| map_sdk_error(e, "change_message_visibility"))?;
        Ok(())
    }
}
