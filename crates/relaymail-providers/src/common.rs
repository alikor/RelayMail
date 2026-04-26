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

#[cfg(test)]
mod tests {
    use relaymail_delivery::SendError;
    use reqwest::StatusCode;
    use serde_json::json;

    use super::*;

    #[test]
    fn endpoint_trims_base_slash_and_keeps_path() {
        assert_eq!(
            endpoint("https://api.example.test/", "/email/send"),
            "https://api.example.test/email/send"
        );
    }

    #[test]
    fn http_statuses_map_to_normalized_errors() {
        let throttled = map_http_error("provider", StatusCode::TOO_MANY_REQUESTS, "");
        assert!(matches!(throttled, SendError::Throttled(_)));

        let timeout = map_http_error("provider", StatusCode::REQUEST_TIMEOUT, "later");
        assert!(matches!(timeout, SendError::Transient(_)));

        let server = map_http_error("provider", StatusCode::BAD_GATEWAY, "down");
        assert!(matches!(server, SendError::Transient(_)));

        let auth = map_http_error("provider", StatusCode::UNAUTHORIZED, "nope");
        assert!(matches!(auth, SendError::AuthenticationFailure(_)));

        let conflict = map_http_error(
            "provider",
            StatusCode::CONFLICT,
            "concurrent_idempotent_requests",
        );
        assert!(matches!(conflict, SendError::Transient(_)));

        let validation = map_http_error("provider", StatusCode::BAD_REQUEST, "bad payload");
        assert!(matches!(validation, SendError::Validation(_)));

        let permanent = map_http_error("provider", StatusCode::MULTIPLE_CHOICES, "odd");
        assert!(matches!(permanent, SendError::Permanent(_)));
    }

    #[test]
    fn provider_body_redacts_by_truncation() {
        let body = "x".repeat(600);
        let mapped = provider_body("provider", StatusCode::BAD_REQUEST, &body);
        assert!(mapped.starts_with("provider: HTTP 400 Bad Request: "));
        assert!(mapped.len() < body.len());
    }

    #[test]
    fn json_string_and_safe_ascii_filter_values() {
        let value = json!({"id":"abc","other":42});
        assert_eq!(json_string(&value, "id").as_deref(), Some("abc"));
        assert_eq!(json_string(&value, "other"), None);
        assert_eq!(
            safe_ascii("Campaign 01!_ok", 20).as_deref(),
            Some("Campaign01_ok")
        );
        assert_eq!(safe_ascii("🙃", 20), None);
        assert_eq!(safe_ascii("abcdef", 3).as_deref(), Some("abc"));
    }
}
