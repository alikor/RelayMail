use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use relaymail_delivery::EmailEventType;
use serde_json::Value;
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

use crate::http::handlers::AppState;
use crate::metrics_init::emit::{
    emit_webhook_duplicate, emit_webhook_received, emit_webhook_suppression,
};
use crate::transport::{
    normalize_email, EmailEventRecord, EventRecordStatus, SuppressionRecord, TransportStore,
};

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone, Debug, Default)]
pub struct WebhookAuthConfig {
    pub resend_secret: Option<String>,
    pub postmark_username: Option<String>,
    pub postmark_password: Option<String>,
    pub smtp2go_auth_token: Option<String>,
}

#[derive(Clone, Debug)]
pub struct WebhookConfig {
    pub auth: WebhookAuthConfig,
    pub store_raw_payloads: bool,
}

#[derive(Clone, Debug)]
pub struct WebhookState {
    pub config: WebhookConfig,
    pub store: Arc<dyn TransportStore>,
}

pub(crate) async fn resend_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    handle_provider_webhook("resend", state, headers, body).await
}

pub(crate) async fn postmark_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    handle_provider_webhook("postmark", state, headers, body).await
}

pub(crate) async fn smtp2go_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    handle_provider_webhook("smtp2go", state, headers, body).await
}

async fn handle_provider_webhook(
    provider: &str,
    state: AppState,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let Some(webhooks) = state.webhooks else {
        return (StatusCode::NOT_FOUND, "webhooks disabled\n");
    };
    if !authenticate(provider, &webhooks.config.auth, &headers, &body) {
        return (StatusCode::UNAUTHORIZED, "unauthorized\n");
    }
    let payload: Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid json\n"),
    };
    let event = normalize_event(
        provider,
        &headers,
        &payload,
        &body,
        webhooks.config.store_raw_payloads,
    );
    emit_webhook_received(provider, event_type_label(&event.event_type));
    let should_suppress = suppresses(&event.event_type);
    let status = match webhooks.store.record_event(event.clone()).await {
        Ok(status) => status,
        Err(_) => return (StatusCode::SERVICE_UNAVAILABLE, "store unavailable\n"),
    };
    if status == EventRecordStatus::Duplicate {
        emit_webhook_duplicate(provider);
        return (StatusCode::OK, "duplicate\n");
    }
    if should_suppress {
        if let Some(recipient) = event.recipient.as_deref() {
            let _ = webhooks
                .store
                .suppress(SuppressionRecord {
                    email_address_normalized: normalize_email(recipient),
                    stream: event.stream.clone(),
                    reason: event_type_label(&event.event_type).into(),
                    source_provider: Some(provider.into()),
                    source_event_id: event.provider_event_id.clone(),
                    created_at_utc: Utc::now(),
                    expires_at_utc: None,
                    notes: None,
                })
                .await;
            emit_webhook_suppression(provider, event_type_label(&event.event_type));
        }
    }
    (StatusCode::OK, "ok\n")
}

fn authenticate(
    provider: &str,
    auth: &WebhookAuthConfig,
    headers: &HeaderMap,
    body: &[u8],
) -> bool {
    match provider {
        "resend" => auth
            .resend_secret
            .as_deref()
            .is_some_and(|secret| verify_svix(secret, headers, body)),
        "postmark" => match (&auth.postmark_username, &auth.postmark_password) {
            (Some(user), Some(pass)) => verify_basic(headers, user, pass),
            _ => false,
        },
        "smtp2go" => auth
            .smtp2go_auth_token
            .as_deref()
            .is_some_and(|token| verify_bearer(headers, token)),
        _ => false,
    }
}

fn verify_svix(secret: &str, headers: &HeaderMap, body: &[u8]) -> bool {
    let Some(id) = header(headers, "svix-id") else {
        return false;
    };
    let Some(timestamp) = header(headers, "svix-timestamp") else {
        return false;
    };
    let Some(signature) = header(headers, "svix-signature") else {
        return false;
    };
    let key = secret
        .strip_prefix("whsec_")
        .and_then(|s| STANDARD.decode(s).ok())
        .unwrap_or_else(|| secret.as_bytes().to_vec());
    let mut mac = match HmacSha256::new_from_slice(&key) {
        Ok(mac) => mac,
        Err(_) => return false,
    };
    mac.update(format!("{id}.{timestamp}.").as_bytes());
    mac.update(body);
    let expected = STANDARD.encode(mac.finalize().into_bytes());
    signature.split_whitespace().any(|part| {
        part.strip_prefix("v1,")
            .is_some_and(|got| got.as_bytes().ct_eq(expected.as_bytes()).into())
    })
}

fn verify_basic(headers: &HeaderMap, username: &str, password: &str) -> bool {
    let Some(auth) = header(headers, "authorization") else {
        return false;
    };
    let Some(encoded) = auth.strip_prefix("Basic ") else {
        return false;
    };
    let expected = format!("{username}:{password}");
    STANDARD
        .decode(encoded)
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .is_some_and(|got| got.as_bytes().ct_eq(expected.as_bytes()).into())
}

fn verify_bearer(headers: &HeaderMap, token: &str) -> bool {
    header(headers, "authorization")
        .and_then(|auth| auth.strip_prefix("Bearer ").map(str::to_string))
        .is_some_and(|got| got.as_bytes().ct_eq(token.as_bytes()).into())
}

fn normalize_event(
    provider: &str,
    headers: &HeaderMap,
    payload: &Value,
    raw_body: &[u8],
    store_raw_payloads: bool,
) -> EmailEventRecord {
    let event_type = provider_event_type(provider, payload);
    let provider_event_id = match provider {
        "resend" => header(headers, "svix-id").or_else(|| json_str(payload, "id")),
        "postmark" => json_str(payload, "ID").or_else(|| json_str(payload, "MessageID")),
        "smtp2go" => json_str(payload, "id").or_else(|| json_str(payload, "email_id")),
        _ => None,
    };
    let provider_message_id = json_str(payload, "MessageID")
        .or_else(|| json_str(payload, "message_id"))
        .or_else(|| json_str(payload, "email_id"))
        .or_else(|| nested_str(payload, &["data", "email_id"]));
    let recipient = json_str(payload, "Recipient")
        .or_else(|| json_str(payload, "Email"))
        .or_else(|| json_str(payload, "rcpt"))
        .or_else(|| nested_str(payload, &["data", "to"]).map(first_arrayish));
    let occurred_at_utc = json_str(payload, "ReceivedAt")
        .or_else(|| json_str(payload, "DeliveredAt"))
        .or_else(|| json_str(payload, "time"))
        .or_else(|| nested_str(payload, &["created_at"]))
        .and_then(|v| DateTime::parse_from_rfc3339(&v).ok())
        .map(|v| v.with_timezone(&Utc));
    let deduplication_key = provider_event_id.clone().unwrap_or_else(|| {
        hash_key(&format!(
            "{provider}|{:?}|{:?}|{:?}|{:?}",
            provider_message_id, event_type, recipient, occurred_at_utc
        ))
    });
    EmailEventRecord {
        provider: provider.into(),
        provider_event_id,
        provider_message_id,
        internal_message_id: nested_str(payload, &["metadata", "internal_message_id"]),
        recipient,
        stream: nested_str(payload, &["metadata", "stream"]),
        event_type,
        occurred_at_utc,
        received_at_utc: Utc::now(),
        raw_payload: store_raw_payloads.then(|| String::from_utf8_lossy(raw_body).to_string()),
        deduplication_key,
    }
}

fn provider_event_type(provider: &str, payload: &Value) -> EmailEventType {
    let raw = match provider {
        "resend" => json_str(payload, "type"),
        "postmark" => json_str(payload, "RecordType"),
        "smtp2go" => json_str(payload, "event"),
        _ => None,
    }
    .unwrap_or_default()
    .to_ascii_lowercase();
    match raw.as_str() {
        "email.sent" | "sent" | "processed" => EmailEventType::Sent,
        "email.delivered" | "delivery" | "delivered" => EmailEventType::Delivered,
        "email.delivery_delayed" | "deliverydelayed" | "delayed" => EmailEventType::DeliveryDelayed,
        "email.bounced" | "bounce" | "hard-bounced" => EmailEventType::PermanentBounce,
        "soft-bounced" | "transientbounce" => EmailEventType::TransientBounce,
        "email.complained" | "spamcomplaint" | "spam" | "complaint" => EmailEventType::Complaint,
        "email.failed" | "failed" => EmailEventType::Failed,
        "email.opened" | "open" => EmailEventType::Opened,
        "email.clicked" | "click" => EmailEventType::Clicked,
        "email.unsubscribed" | "unsubscribe" => EmailEventType::Unsubscribed,
        "reject" | "rejected" => EmailEventType::Rejected,
        "suppressed" => EmailEventType::Suppressed,
        _ => {
            if raw.contains("bounce") {
                EmailEventType::PermanentBounce
            } else {
                EmailEventType::Unknown
            }
        }
    }
}

fn suppresses(event: &EmailEventType) -> bool {
    matches!(
        event,
        EmailEventType::PermanentBounce
            | EmailEventType::Complaint
            | EmailEventType::Suppressed
            | EmailEventType::Unsubscribed
    )
}

fn event_type_label(event: &EmailEventType) -> &'static str {
    match event {
        EmailEventType::Sent => "sent",
        EmailEventType::Accepted => "accepted",
        EmailEventType::Delivered => "delivered",
        EmailEventType::DeliveryDelayed => "delivery_delayed",
        EmailEventType::PermanentBounce => "permanent_bounce",
        EmailEventType::TransientBounce => "transient_bounce",
        EmailEventType::Complaint => "complaint",
        EmailEventType::Rejected => "rejected",
        EmailEventType::Failed => "failed",
        EmailEventType::Suppressed => "suppressed",
        EmailEventType::Opened => "opened",
        EmailEventType::Clicked => "clicked",
        EmailEventType::Unsubscribed => "unsubscribed",
        EmailEventType::Unknown => "unknown",
    }
}

fn header(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(str::to_string)
}

fn json_str(value: &Value, key: &str) -> Option<String> {
    value.get(key).and_then(Value::as_str).map(str::to_string)
}

fn nested_str(value: &Value, path: &[&str]) -> Option<String> {
    let mut cur = value;
    for key in path {
        cur = cur.get(*key)?;
    }
    match cur {
        Value::String(s) => Some(s.clone()),
        Value::Array(values) => values.first().and_then(Value::as_str).map(str::to_string),
        _ => None,
    }
}

fn first_arrayish(value: String) -> String {
    value
        .trim_matches(|c| matches!(c, '[' | ']' | '"'))
        .split(',')
        .next()
        .unwrap_or(&value)
        .trim()
        .trim_matches('"')
        .to_string()
}

fn hash_key(value: &str) -> String {
    let digest = Sha256::digest(value.as_bytes());
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use axum::http::HeaderValue;

    use super::*;
    use crate::transport::InMemoryTransportStore;

    #[test]
    fn basic_auth_rejects_wrong_password() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            HeaderValue::from_str(&format!("Basic {}", STANDARD.encode("u:bad"))).unwrap(),
        );
        assert!(!verify_basic(&headers, "u", "p"));
    }

    #[tokio::test]
    async fn duplicate_event_is_detected() {
        let store = Arc::new(InMemoryTransportStore::new());
        let state = WebhookState {
            config: WebhookConfig {
                auth: WebhookAuthConfig {
                    smtp2go_auth_token: Some("tok".into()),
                    ..WebhookAuthConfig::default()
                },
                store_raw_payloads: false,
            },
            store,
        };
        let app_state = AppState {
            readiness: Arc::new(crate::ReadinessTracker::new()),
            prometheus: Arc::new(
                metrics_exporter_prometheus::PrometheusBuilder::new()
                    .build_recorder()
                    .handle(),
            ),
            webhooks: Some(Arc::new(state)),
        };
        let mut headers = HeaderMap::new();
        headers.insert("authorization", HeaderValue::from_static("Bearer tok"));
        let body = Bytes::from_static(
            br#"{"id":"evt-1","event":"spam","rcpt":"user@example.net","email_id":"m1"}"#,
        );
        let first =
            handle_provider_webhook("smtp2go", app_state.clone(), headers.clone(), body.clone())
                .await
                .into_response();
        let second = handle_provider_webhook("smtp2go", app_state, headers, body)
            .await
            .into_response();
        assert_eq!(first.status(), StatusCode::OK);
        assert_eq!(second.status(), StatusCode::OK);
    }
}
