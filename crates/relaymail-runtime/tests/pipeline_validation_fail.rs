mod common;

use bytes::Bytes;
use relaymail_core::RawEnvelope;
use relaymail_runtime::pipeline::{process_envelope, PipelineOutcome};
use relaymail_testing::SenderScript;

#[tokio::test]
async fn invalid_email_marks_failure_and_taggs() {
    let f = common::fixture(
        b"Not-A-Valid-Email: garbage\r\n\r\n",
        SenderScript::AlwaysSuccess,
        common::config(false),
    );
    let envelope = RawEnvelope::new("env-bad", Bytes::from_static(b"{}"), "h-bad");

    let outcomes = process_envelope(&f.ctx, &envelope).await;
    match &outcomes[0] {
        PipelineOutcome::Failed {
            classification_label,
            ..
        } => {
            assert_eq!(*classification_label, "validation");
        }
        other => panic!("expected Failed, got {other:?}"),
    }
    assert_eq!(f.email_sender.sent_count(), 0, "no send attempted");
    assert_eq!(f.object_store.tag_records().len(), 1, "failure tagged");
}
