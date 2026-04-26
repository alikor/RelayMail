use std::fmt;
use std::time::Duration;

use async_trait::async_trait;
use relaymail_delivery::{EmailSender, ProviderCapabilities, SendError, SendRequest, SendResult};
use serde::Serialize;
use serde_json::{json, Value};

use crate::common;

#[derive(Clone)]
pub struct ResendConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout: Duration,
}

impl fmt::Debug for ResendConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResendConfig")
            .field("api_key", &"<redacted>")
            .field("base_url", &self.base_url)
            .field("timeout", &self.timeout)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct ResendSender {
    client: reqwest::Client,
    config: ResendConfig,
}

impl ResendSender {
    pub fn new(config: ResendConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
        }
    }
}

#[async_trait]
impl EmailSender for ResendSender {
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities::resend()
    }

    async fn send(&self, request: SendRequest) -> Result<SendResult, SendError> {
        let email = common::normalized(&request)?;
        let payload = build_payload(email);
        let response = self
            .client
            .post(common::endpoint(&self.config.base_url, "/emails"))
            .bearer_auth(&self.config.api_key)
            .header("User-Agent", "relaymail/0.1")
            .header("Idempotency-Key", &email.idempotency_key)
            .timeout(self.config.timeout)
            .json(&payload)
            .send()
            .await
            .map_err(|e| common::map_reqwest("resend", e))?;
        let (status, body) = common::response_body(response).await;
        if !status.is_success() {
            return Err(common::map_http_error("resend", status, &body));
        }
        parse_response(&body)
    }
}

#[derive(Serialize)]
struct ResendAttachment {
    filename: String,
    content: String,
}

pub(crate) fn build_payload(email: &relaymail_delivery::EmailSendRequest) -> Value {
    let mut payload = json!({
        "from": email.from.as_ref().map(|v| v.to_header_value()).unwrap_or_default(),
        "to": common::addresses(&email.to),
        "subject": email.subject.clone().unwrap_or_default(),
    });
    set_array(&mut payload, "cc", common::addresses(&email.cc));
    set_array(&mut payload, "bcc", common::addresses(&email.bcc));
    set_opt(&mut payload, "html", email.html_body.clone());
    set_opt(&mut payload, "text", email.text_body.clone());
    set_opt(
        &mut payload,
        "reply_to",
        email.reply_to.as_ref().map(|v| v.to_header_value()),
    );
    if !email.custom_headers.is_empty() {
        payload["headers"] = json!(email.custom_headers);
    }
    let tags = email
        .metadata
        .iter()
        .filter_map(|(name, value)| {
            Some(json!({
                "name": common::safe_ascii(name, 256)?,
                "value": common::safe_ascii(value, 256)?,
            }))
        })
        .collect::<Vec<_>>();
    if !tags.is_empty() {
        payload["tags"] = Value::Array(tags);
    }
    if !email.attachments.is_empty() {
        let attachments = email
            .attachments
            .iter()
            .map(|a| ResendAttachment {
                filename: a.filename.clone(),
                content: a.content_base64.clone(),
            })
            .collect::<Vec<_>>();
        payload["attachments"] = json!(attachments);
    }
    payload
}

pub(crate) fn parse_response(body: &str) -> Result<SendResult, SendError> {
    let value: Value = serde_json::from_str(body)
        .map_err(|e| SendError::Transient(format!("resend JSON: {e}")))?;
    let id = common::json_string(&value, "id")
        .ok_or_else(|| SendError::Transient("resend response missing id".into()))?;
    Ok(common::send_result(id, None))
}

fn set_opt(payload: &mut Value, key: &str, value: Option<String>) {
    if let Some(value) = value.filter(|v| !v.is_empty()) {
        payload[key] = Value::String(value);
    }
}

fn set_array(payload: &mut Value, key: &str, values: Vec<String>) {
    if !values.is_empty() {
        payload[key] = json!(values);
    }
}

#[cfg(test)]
mod tests {
    use relaymail_core::{IdempotencyKey, MessageId, ObjectId};
    use relaymail_delivery::{EmailAddress, EmailSendRequest};
    use std::collections::BTreeMap;

    use super::*;

    fn sample() -> EmailSendRequest {
        EmailSendRequest {
            internal_message_id: MessageId::new(),
            correlation_id: None,
            stream: "transactional".into(),
            category: Some("welcome".into()),
            from: Some(EmailAddress::new("sender@mail.example.com")),
            reply_to: None,
            to: vec![EmailAddress::new("to@example.net")],
            cc: vec![],
            bcc: vec![],
            subject: Some("Hi".into()),
            html_body: Some("<p>Hi</p>".into()),
            text_body: None,
            template_key: None,
            template_data: BTreeMap::new(),
            attachments: vec![],
            custom_headers: BTreeMap::new(),
            metadata: BTreeMap::from([("stream".into(), "transactional".into())]),
            consent_metadata: BTreeMap::new(),
            unsubscribe_url: None,
            idempotency_key: IdempotencyKey::compute(None, &ObjectId::new("b", "k"), "e", 1)
                .as_str()
                .into(),
            tenant: None,
        }
    }

    #[test]
    fn payload_maps_core_fields() {
        let payload = build_payload(&sample());
        assert_eq!(payload["from"], "sender@mail.example.com");
        assert_eq!(payload["to"][0], "to@example.net");
        assert_eq!(payload["subject"], "Hi");
        assert_eq!(payload["html"], "<p>Hi</p>");
    }

    #[test]
    fn response_extracts_id() {
        let result = parse_response(r#"{"id":"email-123"}"#).unwrap();
        assert_eq!(result.provider_message_id(), "email-123");
    }
}
