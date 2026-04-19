mod common;

use bytes::Bytes;
use relaymail_core::RawEnvelope;
use relaymail_runtime::pipeline::{process_envelope, PipelineOutcome};
use relaymail_testing::SenderScript;

const BASIC: &[u8] = include_bytes!("../../../examples/eml/basic.eml");

#[tokio::test]
async fn second_attempt_is_skipped_as_already_claimed() {
    let f = common::fixture(BASIC, SenderScript::AlwaysSuccess, common::config(false));
    let envelope = RawEnvelope::new("env-a", Bytes::from_static(b"{}"), "h-a");

    // First run sends.
    let first = process_envelope(&f.ctx, &envelope).await;
    assert!(matches!(first[0], PipelineOutcome::Sent { .. }));

    // Second run claims are rejected with `AlreadySent`.
    let second = process_envelope(&f.ctx, &envelope).await;
    assert_eq!(second[0], PipelineOutcome::SkippedAlreadyClaimed);
    assert_eq!(f.email_sender.sent_count(), 1, "no second send");
}
