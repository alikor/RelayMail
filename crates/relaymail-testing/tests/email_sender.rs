use chrono::Utc;
use relaymail_core::{IdempotencyKey, ObjectId};
use relaymail_delivery::{EmailSender, SendError, SendRequest, SendResult};
use relaymail_email::{parse_headers_only, EmailMetadata, RawEmail};
use relaymail_testing::{FakeEmailSender, SenderScript, Step};

const BASIC: &[u8] = include_bytes!("../../../examples/eml/basic.eml");

fn request() -> SendRequest {
    let raw = RawEmail::from_slice(BASIC);
    let headers = parse_headers_only(BASIC).unwrap();
    let meta = EmailMetadata::from_headers(&headers, BASIC.len() as u64);
    let key = IdempotencyKey::compute(None, &ObjectId::new("b", "k.eml"), "e", 1);
    SendRequest::new(raw, meta, key)
}

#[tokio::test]
async fn capabilities_returns_fake_label() {
    let s = FakeEmailSender::new(SenderScript::AlwaysSuccess);
    let c = s.capabilities();
    assert_eq!(c.provider_label, "fake");
    assert!(c.supports_raw_mime && c.supports_custom_headers);
    assert_eq!(c.max_message_bytes, u64::MAX);
}

#[tokio::test]
async fn always_fail_records_request_and_errors() {
    let s = FakeEmailSender::new(SenderScript::AlwaysFail(SendError::Throttled("t".into())));
    assert!(s.send(request()).await.is_err());
    assert_eq!(s.sent_count(), 1, "request logged even on error");
}

#[tokio::test]
async fn always_fail_quota_exceeded() {
    let s = FakeEmailSender::new(SenderScript::AlwaysFail(SendError::QuotaExceeded(
        "q".into(),
    )));
    assert!(s.send(request()).await.is_err());
}

#[tokio::test]
async fn always_fail_validation_error() {
    let s = FakeEmailSender::new(SenderScript::AlwaysFail(SendError::Validation("v".into())));
    assert!(s.send(request()).await.is_err());
}

#[tokio::test]
async fn always_fail_auth_failure() {
    let s = FakeEmailSender::new(SenderScript::AlwaysFail(SendError::AuthenticationFailure(
        "a".into(),
    )));
    assert!(s.send(request()).await.is_err());
}

#[tokio::test]
async fn always_fail_invalid_recipient() {
    let s = FakeEmailSender::new(SenderScript::AlwaysFail(SendError::InvalidRecipient(
        "r@bad".into(),
    )));
    assert!(s.send(request()).await.is_err());
}

#[tokio::test]
async fn always_fail_transient() {
    let s = FakeEmailSender::new(SenderScript::AlwaysFail(SendError::Transient("net".into())));
    assert!(s.send(request()).await.is_err());
}

#[tokio::test]
async fn always_fail_permanent() {
    let s = FakeEmailSender::new(SenderScript::AlwaysFail(SendError::Permanent("p".into())));
    assert!(s.send(request()).await.is_err());
}

#[tokio::test]
async fn sequence_empty_falls_back_to_success() {
    let s = FakeEmailSender::new(SenderScript::sequence(vec![]));
    assert!(s.send(request()).await.is_ok());
}

#[tokio::test]
async fn sequence_consumes_steps_in_order() {
    let steps = vec![
        Step::Fail(SendError::Throttled("t".into())),
        Step::Success(SendResult::new("msg-1", Utc::now())),
    ];
    let s = FakeEmailSender::new(SenderScript::sequence(steps));
    assert!(s.send(request()).await.is_err(), "first step: fail");
    assert!(s.send(request()).await.is_ok(), "second step: success");
    assert_eq!(s.sent_count(), 2);
}

#[tokio::test]
async fn sent_requests_accumulates_across_sends() {
    let s = FakeEmailSender::new(SenderScript::AlwaysSuccess);
    s.send(request()).await.unwrap();
    s.send(request()).await.unwrap();
    assert_eq!(s.sent_requests().len(), 2);
    assert_eq!(s.sent_count(), 2);
}
