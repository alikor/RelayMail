use std::sync::Arc;

use bytes::Bytes;
use relaymail_core::{Clock, DispositionPolicy, ObjectId, ObjectMetadata};
use relaymail_runtime::pipeline::{
    EventParser, FailureDispositionMode, PipelineCtx, ProcessingConfig, SuccessDispositionMode,
};
use relaymail_testing::{
    FakeClock, FakeEmailSender, FakeIdempotencyStore, FakeMessageSource, FakeObjectStore,
    SenderScript,
};

use super::parser::{object_ref, StaticEventParser, BUCKET, KEY};

#[allow(dead_code)]
pub struct Fixture {
    pub object_store: Arc<FakeObjectStore>,
    pub message_source: Arc<FakeMessageSource>,
    pub idempotency_store: Arc<FakeIdempotencyStore>,
    pub email_sender: Arc<FakeEmailSender>,
    pub ctx: PipelineCtx,
}

pub fn config(dry_run: bool) -> ProcessingConfig {
    ProcessingConfig {
        service_name: "relaymail-email-ses".into(),
        provider_label: "ses".into(),
        bucket_allowlist: vec![BUCKET.into()],
        prefix_allowlist: vec!["incoming/".into()],
        supported_extensions: vec![".eml".into(), ".emi".into()],
        max_object_size_bytes: 10 * 1024 * 1024,
        success_mode: SuccessDispositionMode::Tag,
        failure_mode: FailureDispositionMode::Tag,
        success_prefix: "processed/".into(),
        failure_prefix: "failed/".into(),
        delete_unsupported_messages: true,
        delete_invalid_email_messages: true,
        dry_run,
        idempotency_ttl_seconds: 3600,
    }
}

pub fn fixture(body: &[u8], script: SenderScript, cfg: ProcessingConfig) -> Fixture {
    let object_store: Arc<FakeObjectStore> = Arc::new(FakeObjectStore::new());
    object_store.put(
        ObjectId::new(BUCKET, KEY),
        Bytes::copy_from_slice(body),
        ObjectMetadata::new("etag-v1", body.len() as u64),
    );
    let source: Arc<FakeMessageSource> = Arc::new(FakeMessageSource::new());
    let idemp: Arc<FakeIdempotencyStore> = Arc::new(FakeIdempotencyStore::new());
    let sender: Arc<FakeEmailSender> = Arc::new(FakeEmailSender::new(script));
    let clock: Arc<dyn Clock> = Arc::new(FakeClock::epoch());
    let parser: Arc<dyn EventParser> = Arc::new(StaticEventParser {
        refs: vec![object_ref()],
    });
    let ctx = PipelineCtx {
        object_store: object_store.clone(),
        message_source: source.clone(),
        idempotency_store: idemp.clone(),
        email_sender: sender.clone(),
        event_parser: parser,
        clock,
        disposition_policy: DispositionPolicy::default(),
        config: Arc::new(cfg),
        tenant_id: None,
    };
    Fixture {
        object_store,
        message_source: source,
        idempotency_store: idemp,
        email_sender: sender,
        ctx,
    }
}
