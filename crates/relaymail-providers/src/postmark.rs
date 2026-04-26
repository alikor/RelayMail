use std::collections::BTreeMap;
use std::fmt;
use std::time::Duration;

use async_trait::async_trait;
use relaymail_delivery::{EmailSender, ProviderCapabilities, SendError, SendRequest, SendResult};
use serde_json::{json, Value};

use crate::common;

#[derive(Clone)]
pub struct PostmarkConfig {
    pub server_token: String,
    pub base_url: String,
    pub timeout: Duration,
    pub message_streams: BTreeMap<String, String>,
}

impl fmt::Debug for PostmarkConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PostmarkConfig")
            .field("server_token", &"<redacted>")
            .field("base_url", &self.base_url)
            .field("timeout", &self.timeout)
            .field("message_streams", &self.message_streams)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct PostmarkSender {
    client: reqwest::Client,
    config: PostmarkConfig,
}

impl PostmarkSender {
    pub fn new(config: PostmarkConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
        }
    }
}

#[async_trait]
impl EmailSender for PostmarkSender {
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities::postmark()
    }

    async fn send(&self, request: SendRequest) -> Result<SendResult, SendError> {
        let email = common::normalized(&request)?;
        let payload = build_payload(email, &self.config.message_streams);
        let response = self
            .client
            .post(common::endpoint(&self.config.base_url, "/email"))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("X-Postmark-Server-Token", &self.config.server_token)
            .timeout(self.config.timeout)
            .json(&payload)
            .send()
            .await
            .map_err(|e| common::map_reqwest("postmark", e))?;
        let (status, body) = common::response_body(response).await;
        if !status.is_success() {
            return Err(common::map_http_error("postmark", status, &body));
        }
        parse_response(&body)
    }
}

pub(crate) fn build_payload(
    email: &relaymail_delivery::EmailSendRequest,
    streams: &BTreeMap<String, String>,
) -> Value {
    let mut payload = json!({
        "From": email.from.as_ref().map(|v| v.to_header_value()).unwrap_or_default(),
        "To": common::joined_addresses(&email.to).unwrap_or_default(),
        "Subject": email.subject.clone().unwrap_or_default(),
        "MessageStream": streams
            .get(&email.stream)
            .cloned()
            .unwrap_or_else(|| "outbound".to_string()),
        "TrackOpens": email.stream == "marketing",
        "TrackLinks": if email.stream == "marketing" { "HtmlAndText" } else { "None" },
    });
    set_opt(&mut payload, "Cc", common::joined_addresses(&email.cc));
    set_opt(&mut payload, "Bcc", common::joined_addresses(&email.bcc));
    set_opt(&mut payload, "HtmlBody", email.html_body.clone());
    set_opt(&mut payload, "TextBody", email.text_body.clone());
    set_opt(
        &mut payload,
        "ReplyTo",
        email.reply_to.as_ref().map(|v| v.to_header_value()),
    );
    if !email.custom_headers.is_empty() {
        payload["Headers"] = Value::Array(
            email
                .custom_headers
                .iter()
                .map(|(name, value)| json!({"Name": name, "Value": value}))
                .collect(),
        );
    }
    if !email.metadata.is_empty() {
        payload["Metadata"] = json!(email.metadata);
    }
    if !email.attachments.is_empty() {
        payload["Attachments"] = Value::Array(
            email
                .attachments
                .iter()
                .map(|a| {
                    let mut item = json!({
                        "Name": a.filename,
                        "Content": a.content_base64,
                        "ContentType": a.content_type,
                    });
                    set_opt(&mut item, "ContentID", a.content_id.clone());
                    item
                })
                .collect(),
        );
    }
    payload
}

pub(crate) fn parse_response(body: &str) -> Result<SendResult, SendError> {
    let value: Value = serde_json::from_str(body)
        .map_err(|e| SendError::Transient(format!("postmark JSON: {e}")))?;
    if value.get("ErrorCode").and_then(Value::as_i64).unwrap_or(0) != 0 {
        let message = value
            .get("Message")
            .and_then(Value::as_str)
            .unwrap_or("postmark rejected message");
        return Err(SendError::Validation(message.to_string()));
    }
    let id = common::json_string(&value, "MessageID")
        .ok_or_else(|| SendError::Transient("postmark response missing MessageID".into()))?;
    Ok(common::send_result(
        id,
        value.get("SubmittedAt").and_then(Value::as_str),
    ))
}

fn set_opt(payload: &mut Value, key: &str, value: Option<String>) {
    if let Some(value) = value.filter(|v| !v.is_empty()) {
        payload[key] = Value::String(value);
    }
}

#[cfg(test)]
mod tests {
    use relaymail_core::{IdempotencyKey, MessageId, ObjectId};
    use relaymail_delivery::{EmailAddress, EmailSendRequest};

    use super::*;

    fn sample() -> EmailSendRequest {
        EmailSendRequest {
            internal_message_id: MessageId::new(),
            correlation_id: None,
            stream: "marketing".into(),
            category: None,
            from: Some(EmailAddress::new("updates@news.example.com")),
            reply_to: Some(EmailAddress::new("reply@example.com")),
            to: vec![EmailAddress::new("to@example.net")],
            cc: vec![],
            bcc: vec![],
            subject: Some("Hi".into()),
            html_body: Some("<p>Hi</p>".into()),
            text_body: Some("Hi".into()),
            template_key: None,
            template_data: BTreeMap::new(),
            attachments: vec![],
            custom_headers: BTreeMap::from([("X-Test".into(), "yes".into())]),
            metadata: BTreeMap::from([("campaign".into(), "c1".into())]),
            consent_metadata: BTreeMap::new(),
            unsubscribe_url: None,
            idempotency_key: IdempotencyKey::compute(None, &ObjectId::new("b", "k"), "e", 1)
                .as_str()
                .into(),
            tenant: None,
        }
    }

    #[test]
    fn payload_uses_stream_message_stream() {
        let streams = BTreeMap::from([("marketing".into(), "broadcasts".into())]);
        let payload = build_payload(&sample(), &streams);
        assert_eq!(payload["MessageStream"], "broadcasts");
        assert_eq!(payload["Headers"][0]["Name"], "X-Test");
        assert_eq!(payload["TrackLinks"], "HtmlAndText");
    }

    #[test]
    fn response_extracts_message_id() {
        let result = parse_response(
            r#"{"ErrorCode":0,"Message":"OK","MessageID":"pm-123","SubmittedAt":"2026-01-01T00:00:00Z"}"#,
        )
        .unwrap();
        assert_eq!(result.provider_message_id(), "pm-123");
    }
}
