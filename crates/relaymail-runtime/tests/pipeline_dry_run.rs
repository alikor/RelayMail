mod common;

use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use relaymail_core::RawEnvelope;
use relaymail_runtime::pipeline::{
    process_envelope, EventParseError, EventParser, ObjectRef, PipelineCtx, PipelineOutcome,
};
use relaymail_testing::SenderScript;

const BASIC: &[u8] = include_bytes!("../../../examples/eml/basic.eml");

#[tokio::test]
async fn dry_run_returns_dry_run_sent_and_skips_send() {
    let f = common::fixture(BASIC, SenderScript::AlwaysSuccess, common::config(true));
    let envelope = RawEnvelope::new("env-dr", Bytes::from_static(b"{}"), "h-dr");

    let outcomes = process_envelope(&f.ctx, &envelope).await;
    assert_eq!(outcomes.len(), 1);
    assert_eq!(outcomes[0], PipelineOutcome::DryRunSent);
    assert_eq!(f.email_sender.sent_count(), 0, "no real send in dry-run");
    assert!(f.object_store.tag_records().is_empty(), "no tag in dry-run");
}

#[derive(Debug)]
struct ErrorParser;

#[async_trait]
impl EventParser for ErrorParser {
    fn parse(&self, _: &[u8]) -> Result<Vec<ObjectRef>, EventParseError> {
        Err(EventParseError::UnknownEnvelope)
    }
}

#[tokio::test]
async fn unknown_envelope_on_parse_error() {
    let f = common::fixture(BASIC, SenderScript::AlwaysSuccess, common::config(false));
    let ctx = PipelineCtx {
        event_parser: Arc::new(ErrorParser),
        ..f.ctx.clone()
    };
    let envelope = RawEnvelope::new("env-unk", Bytes::from_static(b"{}"), "h-unk");

    let outcomes = process_envelope(&ctx, &envelope).await;
    assert_eq!(outcomes.len(), 1);
    assert_eq!(outcomes[0], PipelineOutcome::UnknownEnvelope);
    assert_eq!(f.email_sender.sent_count(), 0);
}
