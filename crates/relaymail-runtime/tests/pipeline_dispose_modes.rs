mod common;

use bytes::Bytes;
use relaymail_core::RawEnvelope;
use relaymail_runtime::pipeline::{
    process_envelope, FailureDispositionMode, PipelineOutcome, SuccessDispositionMode,
};
use relaymail_testing::SenderScript;

const BASIC: &[u8] = include_bytes!("../../../examples/eml/basic.eml");
const INVALID: &[u8] = b"Not-A-Valid-Email: garbage\r\n\r\n";

#[tokio::test]
async fn success_mode_move_relocates_object() {
    let mut cfg = common::config(false);
    cfg.success_mode = SuccessDispositionMode::Move;
    let f = common::fixture(BASIC, SenderScript::AlwaysSuccess, cfg);
    let env = RawEnvelope::new("e1", Bytes::from_static(b"{}"), "h1");

    let outcomes = process_envelope(&f.ctx, &env).await;
    assert!(matches!(outcomes[0], PipelineOutcome::Sent { .. }));
    assert_eq!(f.object_store.moves().len(), 1, "object moved on success");
    assert!(
        f.object_store.tag_records().is_empty(),
        "no tag when mode=Move"
    );
    assert!(f.object_store.deletes().is_empty());
}

#[tokio::test]
async fn success_mode_delete_removes_object() {
    let mut cfg = common::config(false);
    cfg.success_mode = SuccessDispositionMode::Delete;
    let f = common::fixture(BASIC, SenderScript::AlwaysSuccess, cfg);
    let env = RawEnvelope::new("e2", Bytes::from_static(b"{}"), "h2");

    let outcomes = process_envelope(&f.ctx, &env).await;
    assert!(matches!(outcomes[0], PipelineOutcome::Sent { .. }));
    assert_eq!(
        f.object_store.deletes().len(),
        1,
        "object deleted on success"
    );
    assert!(f.object_store.tag_records().is_empty());
    assert!(f.object_store.moves().is_empty());
}

#[tokio::test]
async fn success_mode_none_leaves_object_untouched() {
    let mut cfg = common::config(false);
    cfg.success_mode = SuccessDispositionMode::None;
    let f = common::fixture(BASIC, SenderScript::AlwaysSuccess, cfg);
    let env = RawEnvelope::new("e3", Bytes::from_static(b"{}"), "h3");

    let outcomes = process_envelope(&f.ctx, &env).await;
    assert!(matches!(outcomes[0], PipelineOutcome::Sent { .. }));
    assert!(f.object_store.tag_records().is_empty());
    assert!(f.object_store.moves().is_empty());
    assert!(f.object_store.deletes().is_empty());
}

#[tokio::test]
async fn failure_mode_move_relocates_on_permanent_fail() {
    let mut cfg = common::config(false);
    cfg.failure_mode = FailureDispositionMode::Move;
    let f = common::fixture(INVALID, SenderScript::AlwaysSuccess, cfg);
    let env = RawEnvelope::new("e4", Bytes::from_static(b"{}"), "h4");

    let outcomes = process_envelope(&f.ctx, &env).await;
    assert!(
        matches!(outcomes[0], PipelineOutcome::Failed { .. }),
        "validation fail is permanent"
    );
    assert_eq!(
        f.object_store.moves().len(),
        1,
        "object moved on perm failure"
    );
    assert!(
        f.object_store.tag_records().is_empty(),
        "no tag when mode=Move"
    );
}

#[tokio::test]
async fn failure_mode_none_leaves_object_on_perm_fail() {
    let mut cfg = common::config(false);
    cfg.failure_mode = FailureDispositionMode::None;
    let f = common::fixture(INVALID, SenderScript::AlwaysSuccess, cfg);
    let env = RawEnvelope::new("e5", Bytes::from_static(b"{}"), "h5");

    let outcomes = process_envelope(&f.ctx, &env).await;
    assert!(matches!(outcomes[0], PipelineOutcome::Failed { .. }));
    assert!(f.object_store.tag_records().is_empty());
    assert!(f.object_store.moves().is_empty());
}
