mod common;

use bytes::Bytes;
use relaymail_core::RawEnvelope;
use relaymail_runtime::pipeline::{process_envelope, PipelineOutcome};
use relaymail_testing::SenderScript;

const BASIC: &[u8] = include_bytes!("../../../examples/eml/basic.eml");

#[tokio::test]
async fn throttle_classifies_as_transient_and_does_not_tag_failed() {
    let f = common::fixture(
        BASIC,
        SenderScript::always_throttled(),
        common::config(false),
    );
    let envelope = RawEnvelope::new("env-throt", Bytes::from_static(b"{}"), "h-throt");

    let outcomes = process_envelope(&f.ctx, &envelope).await;
    match &outcomes[0] {
        PipelineOutcome::Failed {
            classification_label,
            ..
        } => {
            assert_eq!(*classification_label, "transient");
        }
        other => panic!("expected Failed, got {other:?}"),
    }
    // Transient failures must NOT be tagged on the object (retry path).
    assert!(f.object_store.tag_records().is_empty());
}
