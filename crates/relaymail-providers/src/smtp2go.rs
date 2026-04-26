use std::fmt;
use std::time::Duration;

use async_trait::async_trait;
use relaymail_delivery::{EmailSender, ProviderCapabilities, SendError, SendRequest, SendResult};
use serde_json::{json, Value};

use crate::common;

#[derive(Clone)]
pub struct Smtp2GoConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout: Duration,
}

impl fmt::Debug for Smtp2GoConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Smtp2GoConfig")
            .field("api_key", &"<redacted>")
            .field("base_url", &self.base_url)
            .field("timeout", &self.timeout)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct Smtp2GoSender {
    client: reqwest::Client,
    config: Smtp2GoConfig,
}

impl Smtp2GoSender {
    pub fn new(config: Smtp2GoConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
        }
    }
}

#[async_trait]
impl EmailSender for Smtp2GoSender {
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities::smtp2go()
    }

    async fn send(&self, request: SendRequest) -> Result<SendResult, SendError> {
        let email = common::normalized(&request)?;
        let payload = build_payload(email);
        let response = self
            .client
            .post(common::endpoint(&self.config.base_url, "/email/send"))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("X-Smtp2go-Api-Key", &self.config.api_key)
            .timeout(self.config.timeout)
            .json(&payload)
            .send()
            .await
            .map_err(|e| common::map_reqwest("smtp2go", e))?;
        let (status, body) = common::response_body(response).await;
        if !status.is_success() {
            return Err(common::map_http_error("smtp2go", status, &body));
        }
        parse_response(&body)
    }
}

pub(crate) fn build_payload(email: &relaymail_delivery::EmailSendRequest) -> Value {
    let mut payload = json!({
        "sender": email.from.as_ref().map(|v| v.to_header_value()).unwrap_or_default(),
        "to": common::addresses(&email.to),
        "subject": email.subject.clone().unwrap_or_default(),
    });
    set_array(&mut payload, "cc", common::addresses(&email.cc));
    set_array(&mut payload, "bcc", common::addresses(&email.bcc));
    set_opt(&mut payload, "html_body", email.html_body.clone());
    set_opt(&mut payload, "text_body", email.text_body.clone());
    if !email.custom_headers.is_empty() {
        payload["custom_headers"] = Value::Array(
            email
                .custom_headers
                .iter()
                .map(|(header, value)| json!({"header": header, "value": value}))
                .collect(),
        );
    }
    if !email.attachments.is_empty() {
        payload["attachments"] = Value::Array(
            email
                .attachments
                .iter()
                .map(|a| {
                    json!({
                        "filename": a.filename,
                        "fileblob": a.content_base64,
                        "mimetype": a.content_type,
                    })
                })
                .collect(),
        );
    }
    payload
}

pub(crate) fn parse_response(body: &str) -> Result<SendResult, SendError> {
    let value: Value = serde_json::from_str(body)
        .map_err(|e| SendError::Transient(format!("smtp2go JSON: {e}")))?;
    if let Some(error) = value
        .get("data")
        .and_then(|v| v.get("error"))
        .and_then(Value::as_str)
    {
        return Err(SendError::Validation(error.to_string()));
    }
    let response = value
        .get("email_response")
        .ok_or_else(|| SendError::Transient("smtp2go response missing email_response".into()))?;
    let succeeded = response
        .get("succeeded")
        .and_then(Value::as_u64)
        .unwrap_or_default();
    if succeeded == 0 {
        let failures = response
            .get("failures")
            .map(Value::to_string)
            .unwrap_or_else(|| "smtp2go send failed".to_string());
        return Err(SendError::Validation(failures));
    }
    let id = response
        .get("email_id")
        .and_then(Value::as_str)
        .or_else(|| value.get("request_id").and_then(Value::as_str))
        .ok_or_else(|| SendError::Transient("smtp2go response missing email_id".into()))?;
    Ok(common::send_result(id.to_string(), None).with_metadata(
        "request_id",
        value
            .get("request_id")
            .and_then(Value::as_str)
            .unwrap_or_default(),
    ))
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
    use std::collections::BTreeMap;

    use relaymail_core::{IdempotencyKey, MessageId, ObjectId};
    use relaymail_delivery::{EmailAddress, EmailSendRequest};

    use super::*;

    fn sample() -> EmailSendRequest {
        EmailSendRequest {
            internal_message_id: MessageId::new(),
            correlation_id: None,
            stream: "transactional".into(),
            category: None,
            from: Some(EmailAddress::new("sender@mail.example.com")),
            reply_to: None,
            to: vec![EmailAddress::new("to@example.net")],
            cc: vec![],
            bcc: vec![],
            subject: Some("Hi".into()),
            html_body: None,
            text_body: Some("Hi".into()),
            template_key: None,
            template_data: BTreeMap::new(),
            attachments: vec![],
            custom_headers: BTreeMap::new(),
            metadata: BTreeMap::new(),
            consent_metadata: BTreeMap::new(),
            unsubscribe_url: None,
            idempotency_key: IdempotencyKey::compute(None, &ObjectId::new("b", "k"), "e", 1)
                .as_str()
                .into(),
            tenant: None,
        }
    }

    #[test]
    fn payload_maps_required_fields() {
        let payload = build_payload(&sample());
        assert_eq!(payload["sender"], "sender@mail.example.com");
        assert_eq!(payload["to"][0], "to@example.net");
        assert_eq!(payload["text_body"], "Hi");
    }

    #[test]
    fn response_extracts_email_id() {
        let result = parse_response(
            r#"{"email_response":{"succeeded":1,"failed":0,"failures":[],"email_id":"s2g-123"},"request_id":"req-1"}"#,
        )
        .unwrap();
        assert_eq!(result.provider_message_id(), "s2g-123");
    }
}
