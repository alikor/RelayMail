use std::collections::{BTreeMap, HashSet, VecDeque};
use std::fmt;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use relaymail_delivery::{
    EmailAddress, EmailEventType, EmailSendRequest, EmailSender, ProviderCapabilities, SendError,
    SendRequest, SendResult,
};

#[derive(Clone, Debug)]
pub struct StreamPolicy {
    pub name: String,
    pub allowed_from_domains: Vec<String>,
    pub default_from: Option<String>,
    pub default_reply_to: Option<String>,
    pub require_unsubscribe: bool,
    pub require_consent_metadata: bool,
    pub provider_chain: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct DeliveryPolicy {
    pub streams: BTreeMap<String, StreamPolicy>,
    pub default_stream: String,
    pub fallback_enabled: bool,
    pub global_max_per_minute: u32,
}

impl DeliveryPolicy {
    pub fn stream(&self, name: &str) -> Option<&StreamPolicy> {
        self.streams
            .get(&name.to_ascii_lowercase())
            .or_else(|| self.streams.get(&self.default_stream))
    }
}

#[derive(Clone, Debug)]
pub struct SendAttemptRecord {
    pub internal_message_id: String,
    pub provider: String,
    pub attempt_number: u32,
    pub started_at_utc: DateTime<Utc>,
    pub completed_at_utc: DateTime<Utc>,
    pub status: String,
    pub provider_message_id: Option<String>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Clone, Debug)]
pub struct MessageLogRecord {
    pub internal_message_id: String,
    pub correlation_id: Option<String>,
    pub stream: String,
    pub provider: String,
    pub provider_message_id: Option<String>,
    pub status: String,
    pub attempt_count: u32,
    pub created_at_utc: DateTime<Utc>,
    pub accepted_at_utc: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

#[derive(Clone, Debug)]
pub struct EmailEventRecord {
    pub provider: String,
    pub provider_event_id: Option<String>,
    pub provider_message_id: Option<String>,
    pub internal_message_id: Option<String>,
    pub recipient: Option<String>,
    pub stream: Option<String>,
    pub event_type: EmailEventType,
    pub occurred_at_utc: Option<DateTime<Utc>>,
    pub received_at_utc: DateTime<Utc>,
    pub raw_payload: Option<String>,
    pub deduplication_key: String,
}

#[derive(Clone, Debug)]
pub struct SuppressionRecord {
    pub email_address_normalized: String,
    pub stream: Option<String>,
    pub reason: String,
    pub source_provider: Option<String>,
    pub source_event_id: Option<String>,
    pub created_at_utc: DateTime<Utc>,
    pub expires_at_utc: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventRecordStatus {
    Accepted,
    Duplicate,
}

#[derive(Debug, thiserror::Error)]
pub enum TransportStoreError {
    #[error("transport store unavailable: {0}")]
    Unavailable(String),
}

#[async_trait]
pub trait TransportStore: Send + Sync + fmt::Debug {
    async fn is_suppressed(
        &self,
        email_address: &str,
        stream: &str,
    ) -> Result<bool, TransportStoreError>;

    async fn record_send_attempt(
        &self,
        record: SendAttemptRecord,
    ) -> Result<(), TransportStoreError>;

    async fn record_message(&self, record: MessageLogRecord) -> Result<(), TransportStoreError>;

    async fn record_event(
        &self,
        record: EmailEventRecord,
    ) -> Result<EventRecordStatus, TransportStoreError>;

    async fn suppress(&self, record: SuppressionRecord) -> Result<(), TransportStoreError>;
}

#[derive(Debug, Default)]
pub struct InMemoryTransportStore {
    suppressed: Mutex<HashSet<(String, String)>>,
    events: Mutex<HashSet<String>>,
    pub attempts: Mutex<Vec<SendAttemptRecord>>,
    pub messages: Mutex<Vec<MessageLogRecord>>,
}

impl InMemoryTransportStore {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl TransportStore for InMemoryTransportStore {
    async fn is_suppressed(
        &self,
        email_address: &str,
        stream: &str,
    ) -> Result<bool, TransportStoreError> {
        let email = normalize_email(email_address);
        let stream = stream.to_ascii_lowercase();
        let guard = self.suppressed.lock().expect("poisoned");
        Ok(guard.contains(&(email.clone(), stream)) || guard.contains(&(email, "*".into())))
    }

    async fn record_send_attempt(
        &self,
        record: SendAttemptRecord,
    ) -> Result<(), TransportStoreError> {
        self.attempts.lock().expect("poisoned").push(record);
        Ok(())
    }

    async fn record_message(&self, record: MessageLogRecord) -> Result<(), TransportStoreError> {
        self.messages.lock().expect("poisoned").push(record);
        Ok(())
    }

    async fn record_event(
        &self,
        record: EmailEventRecord,
    ) -> Result<EventRecordStatus, TransportStoreError> {
        let mut guard = self.events.lock().expect("poisoned");
        if !guard.insert(record.deduplication_key) {
            return Ok(EventRecordStatus::Duplicate);
        }
        Ok(EventRecordStatus::Accepted)
    }

    async fn suppress(&self, record: SuppressionRecord) -> Result<(), TransportStoreError> {
        self.suppressed.lock().expect("poisoned").insert((
            record.email_address_normalized,
            record
                .stream
                .unwrap_or_else(|| "*".into())
                .to_ascii_lowercase(),
        ));
        Ok(())
    }
}

#[derive(Debug)]
pub struct RelayMailDeliveryService {
    policy: DeliveryPolicy,
    default_chain: Vec<Arc<dyn EmailSender>>,
    stream_chains: BTreeMap<String, Vec<Arc<dyn EmailSender>>>,
    store: Arc<dyn TransportStore>,
    limiter: Mutex<VecDeque<DateTime<Utc>>>,
}

impl RelayMailDeliveryService {
    pub fn new(
        policy: DeliveryPolicy,
        default_chain: Vec<Arc<dyn EmailSender>>,
        stream_chains: BTreeMap<String, Vec<Arc<dyn EmailSender>>>,
        store: Arc<dyn TransportStore>,
    ) -> Self {
        Self {
            policy,
            default_chain,
            stream_chains,
            store,
            limiter: Mutex::new(VecDeque::new()),
        }
    }

    async fn prepare(&self, request: &SendRequest) -> Result<EmailSendRequest, SendError> {
        let mut email = EmailSendRequest::from_raw(
            request.raw(),
            request.idempotency_key(),
            request.tenant().cloned(),
        )?;
        let stream = self
            .policy
            .stream(&email.stream)
            .ok_or_else(|| SendError::Validation(format!("unknown stream `{}`", email.stream)))?;
        email.stream = stream.name.clone();
        if email.from.is_none() {
            email.from = stream.default_from.as_deref().map(parse_address);
        }
        if email.reply_to.is_none() {
            email.reply_to = stream.default_reply_to.as_deref().map(parse_address);
        }
        if stream.require_unsubscribe && !email.custom_headers.contains_key("List-Unsubscribe") {
            if let Some(url) = &email.unsubscribe_url {
                email
                    .custom_headers
                    .insert("List-Unsubscribe".into(), format!("<{url}>"));
            }
        }
        validate_email(&email, stream)?;
        for recipient in email.recipients() {
            if self
                .store
                .is_suppressed(&recipient.address, &email.stream)
                .await
                .map_err(|e| SendError::Transient(e.to_string()))?
            {
                return Err(SendError::Suppressed(recipient.address.clone()));
            }
        }
        self.check_rate_limit()?;
        Ok(email)
    }

    fn check_rate_limit(&self) -> Result<(), SendError> {
        if self.policy.global_max_per_minute == 0 {
            return Ok(());
        }
        let now = Utc::now();
        let cutoff = now - Duration::seconds(60);
        let mut guard = self.limiter.lock().expect("poisoned");
        while guard.front().is_some_and(|ts| *ts < cutoff) {
            guard.pop_front();
        }
        if guard.len() >= self.policy.global_max_per_minute as usize {
            return Err(SendError::Throttled("relaymail global rate limit".into()));
        }
        guard.push_back(now);
        Ok(())
    }

    fn chain_for(&self, stream: &str) -> &[Arc<dyn EmailSender>] {
        self.stream_chains
            .get(stream)
            .filter(|chain| !chain.is_empty())
            .map(Vec::as_slice)
            .unwrap_or(self.default_chain.as_slice())
    }
}

#[async_trait]
impl EmailSender for RelayMailDeliveryService {
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities::relay_chain()
    }

    async fn send(&self, request: SendRequest) -> Result<SendResult, SendError> {
        let email = self.prepare(&request).await?;
        let chain = self.chain_for(&email.stream);
        if chain.is_empty() {
            return Err(SendError::Permanent("no enabled email providers".into()));
        }
        let mut last_error = None;
        for (index, provider) in chain.iter().enumerate() {
            let attempt = index as u32 + 1;
            let provider_name = provider.capabilities().provider_label.to_string();
            let started = Utc::now();
            let provider_request = request.clone().with_email(email.clone());
            match provider.send(provider_request).await {
                Ok(result) => {
                    let result = result.with_metadata("provider", provider_name.clone());
                    let _ = self
                        .store
                        .record_send_attempt(SendAttemptRecord {
                            internal_message_id: email.internal_message_id.to_string(),
                            provider: provider_name.clone(),
                            attempt_number: attempt,
                            started_at_utc: started,
                            completed_at_utc: Utc::now(),
                            status: "accepted".into(),
                            provider_message_id: Some(result.provider_message_id().into()),
                            error_code: None,
                            error_message: None,
                        })
                        .await;
                    let _ = self
                        .store
                        .record_message(MessageLogRecord {
                            internal_message_id: email.internal_message_id.to_string(),
                            correlation_id: email.correlation_id.clone(),
                            stream: email.stream.clone(),
                            provider: provider_name,
                            provider_message_id: Some(result.provider_message_id().into()),
                            status: "accepted".into(),
                            attempt_count: attempt,
                            created_at_utc: started,
                            accepted_at_utc: Some(result.accepted_at()),
                            error_message: None,
                        })
                        .await;
                    return Ok(result);
                }
                Err(err) => {
                    let should_try_next = self.policy.fallback_enabled
                        && index + 1 < chain.len()
                        && is_fallback_safe(&err);
                    let _ = self
                        .store
                        .record_send_attempt(SendAttemptRecord {
                            internal_message_id: email.internal_message_id.to_string(),
                            provider: provider_name,
                            attempt_number: attempt,
                            started_at_utc: started,
                            completed_at_utc: Utc::now(),
                            status: if should_try_next {
                                "retryable"
                            } else {
                                "failed"
                            }
                            .into(),
                            provider_message_id: None,
                            error_code: Some(err.classify().label().into()),
                            error_message: Some(err.to_string()),
                        })
                        .await;
                    if should_try_next {
                        last_error = Some(err);
                        continue;
                    }
                    let _ = self
                        .store
                        .record_message(MessageLogRecord {
                            internal_message_id: email.internal_message_id.to_string(),
                            correlation_id: email.correlation_id.clone(),
                            stream: email.stream.clone(),
                            provider: "relay".into(),
                            provider_message_id: None,
                            status: "failed".into(),
                            attempt_count: attempt,
                            created_at_utc: started,
                            accepted_at_utc: None,
                            error_message: Some(err.to_string()),
                        })
                        .await;
                    return Err(err);
                }
            }
        }
        Err(last_error.unwrap_or_else(|| SendError::Transient("provider chain exhausted".into())))
    }
}

pub fn is_fallback_safe(error: &SendError) -> bool {
    matches!(error, SendError::Transient(_) | SendError::Throttled(_))
}

pub fn normalize_email(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn validate_email(email: &EmailSendRequest, stream: &StreamPolicy) -> Result<(), SendError> {
    if email.recipient_count() == 0 {
        return Err(SendError::Validation(
            "at least one recipient required".into(),
        ));
    }
    if email
        .subject
        .as_deref()
        .unwrap_or_default()
        .trim()
        .is_empty()
    {
        return Err(SendError::Validation("subject required".into()));
    }
    if email.html_body.as_deref().unwrap_or_default().is_empty()
        && email.text_body.as_deref().unwrap_or_default().is_empty()
        && email.template_key.as_deref().unwrap_or_default().is_empty()
    {
        return Err(SendError::Validation(
            "html, text, or template required".into(),
        ));
    }
    let from = email
        .from
        .as_ref()
        .ok_or_else(|| SendError::Validation("from address required".into()))?;
    if !stream.allowed_from_domains.is_empty() {
        let domain = from
            .domain()
            .map(str::to_ascii_lowercase)
            .ok_or_else(|| SendError::Validation("from address missing domain".into()))?;
        if !stream
            .allowed_from_domains
            .iter()
            .any(|allowed| allowed.eq_ignore_ascii_case(&domain))
        {
            return Err(SendError::Validation(format!(
                "from domain `{domain}` is not allowed for stream `{}`",
                stream.name
            )));
        }
    }
    if stream.require_unsubscribe && email.unsubscribe_url.is_none() {
        return Err(SendError::Validation(format!(
            "stream `{}` requires unsubscribe URL",
            stream.name
        )));
    }
    if stream.require_consent_metadata && email.consent_metadata.is_empty() {
        return Err(SendError::Validation(format!(
            "stream `{}` requires consent metadata",
            stream.name
        )));
    }
    Ok(())
}

fn parse_address(value: &str) -> EmailAddress {
    let trimmed = value.trim();
    if let Some((name, rest)) = trimmed.split_once('<') {
        let address = rest.trim_end_matches('>').trim();
        EmailAddress {
            address: address.to_string(),
            display_name: Some(name.trim().trim_matches('"').to_string()).filter(|s| !s.is_empty()),
        }
    } else {
        EmailAddress::new(trimmed)
    }
}

#[cfg(test)]
mod tests {
    use relaymail_core::{IdempotencyKey, ObjectId};
    use relaymail_delivery::{SendRequest, SendResult};
    use relaymail_email::{parse_headers_only, EmailMetadata, RawEmail};

    use super::*;

    fn policy() -> DeliveryPolicy {
        DeliveryPolicy {
            streams: BTreeMap::from([
                (
                    "transactional".into(),
                    StreamPolicy {
                        name: "transactional".into(),
                        allowed_from_domains: vec!["mail.example.com".into()],
                        default_from: None,
                        default_reply_to: None,
                        require_unsubscribe: false,
                        require_consent_metadata: false,
                        provider_chain: vec![],
                    },
                ),
                (
                    "marketing".into(),
                    StreamPolicy {
                        name: "marketing".into(),
                        allowed_from_domains: vec!["news.example.com".into()],
                        default_from: None,
                        default_reply_to: None,
                        require_unsubscribe: true,
                        require_consent_metadata: true,
                        provider_chain: vec![],
                    },
                ),
            ]),
            default_stream: "transactional".into(),
            fallback_enabled: true,
            global_max_per_minute: 0,
        }
    }

    fn request(raw: &[u8]) -> SendRequest {
        let raw = RawEmail::from_slice(raw);
        let headers = parse_headers_only(raw.as_bytes()).unwrap();
        let meta = EmailMetadata::from_headers(&headers, raw.len() as u64);
        let key = IdempotencyKey::compute(None, &ObjectId::new("b", "k.eml"), "e", 1);
        SendRequest::new(raw, meta, key)
    }

    #[derive(Debug)]
    struct OkSender;

    #[async_trait]
    impl EmailSender for OkSender {
        fn capabilities(&self) -> ProviderCapabilities {
            ProviderCapabilities::resend()
        }

        async fn send(&self, _request: SendRequest) -> Result<SendResult, SendError> {
            Ok(SendResult::new("ok", Utc::now()))
        }
    }

    #[derive(Debug)]
    struct FailSender(SendError);

    #[async_trait]
    impl EmailSender for FailSender {
        fn capabilities(&self) -> ProviderCapabilities {
            ProviderCapabilities::postmark()
        }

        async fn send(&self, _request: SendRequest) -> Result<SendResult, SendError> {
            Err(match &self.0 {
                SendError::Throttled(s) => SendError::Throttled(s.clone()),
                SendError::QuotaExceeded(s) => SendError::QuotaExceeded(s.clone()),
                SendError::Validation(s) => SendError::Validation(s.clone()),
                SendError::AuthenticationFailure(s) => SendError::AuthenticationFailure(s.clone()),
                SendError::InvalidRecipient(s) => SendError::InvalidRecipient(s.clone()),
                SendError::Suppressed(s) => SendError::Suppressed(s.clone()),
                SendError::Transient(s) => SendError::Transient(s.clone()),
                SendError::Permanent(s) => SendError::Permanent(s.clone()),
            })
        }
    }

    fn ok_raw() -> &'static [u8] {
        b"From: a@mail.example.com\r\nTo: b@example.net\r\nSubject: x\r\n\r\nbody"
    }

    #[tokio::test]
    async fn rejects_wrong_stream_domain() {
        let service = RelayMailDeliveryService::new(
            policy(),
            vec![Arc::new(OkSender)],
            BTreeMap::new(),
            Arc::new(InMemoryTransportStore::new()),
        );
        let raw = b"From: a@bad.example\r\nTo: b@example.net\r\nSubject: x\r\n\r\nbody";
        let err = service.send(request(raw)).await.unwrap_err();
        assert!(matches!(err, SendError::Validation(_)));
    }

    #[tokio::test]
    async fn marketing_requires_consent_and_unsubscribe() {
        let service = RelayMailDeliveryService::new(
            policy(),
            vec![Arc::new(OkSender)],
            BTreeMap::new(),
            Arc::new(InMemoryTransportStore::new()),
        );
        let raw = b"From: a@news.example.com\r\nTo: b@example.net\r\nSubject: x\r\nX-RelayMail-Stream: marketing\r\n\r\nbody";
        let err = service.send(request(raw)).await.unwrap_err();
        assert!(err.to_string().contains("unsubscribe"));
    }

    #[tokio::test]
    async fn transient_provider_failure_falls_back() {
        let store = Arc::new(InMemoryTransportStore::new());
        let service = RelayMailDeliveryService::new(
            policy(),
            vec![
                Arc::new(FailSender(SendError::Transient("down".into()))),
                Arc::new(OkSender),
            ],
            BTreeMap::new(),
            store.clone(),
        );
        let result = service.send(request(ok_raw())).await.unwrap();
        assert_eq!(result.provider_message_id(), "ok");
        assert_eq!(store.attempts.lock().expect("poisoned").len(), 2);
    }

    #[tokio::test]
    async fn validation_failure_does_not_fallback() {
        let store = Arc::new(InMemoryTransportStore::new());
        let service = RelayMailDeliveryService::new(
            policy(),
            vec![
                Arc::new(FailSender(SendError::Validation("bad".into()))),
                Arc::new(OkSender),
            ],
            BTreeMap::new(),
            store.clone(),
        );
        let err = service.send(request(ok_raw())).await.unwrap_err();
        assert!(matches!(err, SendError::Validation(_)));
        assert_eq!(store.attempts.lock().expect("poisoned").len(), 1);
    }

    #[tokio::test]
    async fn suppression_precheck_blocks_send_attempts() {
        let store = Arc::new(InMemoryTransportStore::new());
        store
            .suppress(SuppressionRecord {
                email_address_normalized: "b@example.net".into(),
                stream: Some("transactional".into()),
                reason: "permanent_bounce".into(),
                source_provider: None,
                source_event_id: None,
                created_at_utc: Utc::now(),
                expires_at_utc: None,
                notes: None,
            })
            .await
            .unwrap();
        let service = RelayMailDeliveryService::new(
            policy(),
            vec![Arc::new(OkSender)],
            BTreeMap::new(),
            store.clone(),
        );
        let err = service.send(request(ok_raw())).await.unwrap_err();
        assert!(matches!(err, SendError::Suppressed(_)));
        assert!(store.attempts.lock().expect("poisoned").is_empty());
    }

    #[test]
    fn only_transient_errors_are_fallback_safe() {
        assert!(is_fallback_safe(&SendError::Transient("x".into())));
        assert!(is_fallback_safe(&SendError::Throttled("x".into())));
        assert!(!is_fallback_safe(&SendError::Validation("x".into())));
    }
}
