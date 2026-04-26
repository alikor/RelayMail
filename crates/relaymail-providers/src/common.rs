use chrono::{DateTime, Utc};
use relaymail_delivery::{EmailAddress, EmailSendRequest, SendError, SendRequest, SendResult};
use serde_json::Value;

pub(crate) fn normalized(request: &SendRequest) -> Result<&EmailSendRequest, SendError> {
    request
        .email()
        .ok_or_else(|| SendError::Validation("normalized email request missing".into()))
}

pub(crate) fn endpoint(base_url: &str, path: &str) -> String {
    format!("{}{}", base_url.trim_end_matches('/'), path)
}

pub(crate) fn addresses(values: &[EmailAddress]) -> Vec<String> {
    values.iter().map(EmailAddress::to_header_value).collect()
}

pub(crate) fn joined_addresses(values: &[EmailAddress]) -> Option<String> {
    if values.is_empty() {
        None
    } else {
        Some(addresses(values).join(", "))
    }
}

pub(crate) fn send_result(provider_message_id: String, accepted_at: Option<&str>) -> SendResult {
    let accepted = accepted_at
        .and_then(|v| DateTime::parse_from_rfc3339(v).ok())
        .map(|v| v.with_timezone(&Utc))
        .unwrap_or_else(Utc::now);
    SendResult::new(provider_message_id, accepted)
}

pub(crate) async fn response_body(response: reqwest::Response) -> (reqwest::StatusCode, String) {
    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    (status, body)
}

pub(crate) fn map_http_error(provider: &str, status: reqwest::StatusCode, body: &str) -> SendError {
    let safe = provider_body(provider, status, body);
    if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
        return SendError::Throttled(safe);
    }
    if status == reqwest::StatusCode::REQUEST_TIMEOUT || status.is_server_error() {
        return SendError::Transient(safe);
    }
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
        return SendError::AuthenticationFailure(safe);
    }
    if status == reqwest::StatusCode::CONFLICT && body.contains("concurrent_idempotent_requests") {
        return SendError::Transient(safe);
    }
    if status.is_client_error() {
        return SendError::Validation(safe);
    }
    SendError::Permanent(safe)
}

pub(crate) fn map_reqwest(provider: &str, err: reqwest::Error) -> SendError {
    if err.is_timeout() || err.is_connect() || err.is_request() {
        SendError::Transient(format!("{provider}: request failed: {err}"))
    } else {
        SendError::Permanent(format!("{provider}: response failed: {err}"))
    }
}

pub(crate) fn provider_body(provider: &str, status: reqwest::StatusCode, body: &str) -> String {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return format!("{provider}: HTTP {status}");
    }
    format!("{provider}: HTTP {status}: {}", truncate(trimmed, 512))
}

pub(crate) fn json_string(value: &Value, key: &str) -> Option<String> {
    value.get(key).and_then(Value::as_str).map(str::to_string)
}

pub(crate) fn safe_ascii(value: &str, max: usize) -> Option<String> {
    let out: String = value
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-'))
        .take(max)
        .collect();
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn truncate(value: &str, max: usize) -> String {
    if value.len() <= max {
        value.to_string()
    } else {
        value[..max].to_string()
    }
}
