use async_trait::async_trait;
use aws_sdk_sesv2::primitives::Blob;
use aws_sdk_sesv2::types::{EmailContent, RawMessage};
use chrono::Utc;
use relaymail_delivery::{EmailSender, ProviderCapabilities, SendError, SendRequest, SendResult};

use super::error_map::map_sdk_error;
use super::sender::SesSender;

#[async_trait]
impl EmailSender for SesSender {
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities::ses_v2()
    }

    async fn send(&self, request: SendRequest) -> Result<SendResult, SendError> {
        let cfg = self.runtime_config();
        let raw = RawMessage::builder()
            .data(Blob::new(request.raw().as_bytes().to_vec()))
            .build()
            .map_err(|e| SendError::Validation(format!("build raw: {e}")))?;
        let content = EmailContent::builder().raw(raw).build();
        let mut call = self.client().send_email().content(content);
        // Per-email override (`X-SES-CONFIGURATION-SET` header) takes
        // precedence over the static runtime default. Producers use
        // this to route individual messages (e.g. transactional vs.
        // future marketing) to different SES configuration sets and
        // thus different event destinations, without a worker rollout.
        let configuration_set = request
            .metadata()
            .configuration_set()
            .or(cfg.configuration_set.as_deref());
        if let Some(cs) = configuration_set {
            call = call.configuration_set_name(cs);
        }
        if let Some(s) = &cfg.source_arn {
            call = call.from_email_address_identity_arn(s);
        }
        if let Some(f) = &cfg.from_arn {
            call = call.feedback_forwarding_email_address_identity_arn(f);
        }
        let out = call.send().await.map_err(map_sdk_error)?;
        Ok(SendResult::new(
            out.message_id.unwrap_or_default(),
            Utc::now(),
        ))
    }
}
