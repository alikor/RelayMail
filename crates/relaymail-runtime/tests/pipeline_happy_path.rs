mod common;

use bytes::Bytes;
use relaymail_core::RawEnvelope;
use relaymail_runtime::pipeline::{process_envelope, PipelineOutcome};
use relaymail_testing::SenderScript;

const BASIC: &[u8] = include_bytes!("../../../examples/eml/basic.eml");

#[tokio::test]
async fn happy_path_sends_and_tags() {
    let f = common::fixture(BASIC, SenderScript::AlwaysSuccess, common::config(false));
    let envelope = RawEnvelope::new("env-1", Bytes::from_static(b"{}"), "handle-1");

    let outcomes = process_envelope(&f.ctx, &envelope).await;
    assert_eq!(outcomes.len(), 1);
    match &outcomes[0] {
        PipelineOutcome::Sent {
            provider_message_id,
        } => assert!(!provider_message_id.is_empty()),
        other => panic!("expected Sent, got {other:?}"),
    }
    assert_eq!(f.email_sender.sent_count(), 1);
    assert_eq!(f.object_store.tag_records().len(), 1);
}
