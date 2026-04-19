mod common;

use bytes::Bytes;
use relaymail_core::{RawEnvelope, TenantId};
use relaymail_runtime::pipeline::{process_envelope, PipelineCtx, PipelineOutcome};
use relaymail_testing::SenderScript;

const BASIC: &[u8] = include_bytes!("../../../examples/eml/basic.eml");

#[tokio::test]
async fn tenant_id_forwarded_to_send_request() {
    let f = common::fixture(BASIC, SenderScript::AlwaysSuccess, common::config(false));
    let tenant = TenantId::parse("acme-corp").unwrap();
    let ctx = PipelineCtx {
        tenant_id: Some(tenant),
        ..f.ctx.clone()
    };
    let envelope = RawEnvelope::new("env-t", Bytes::from_static(b"{}"), "h-t");
    let outcomes = process_envelope(&ctx, &envelope).await;
    assert!(matches!(outcomes[0], PipelineOutcome::Sent { .. }));
    let requests = f.email_sender.sent_requests();
    assert!(requests[0].tenant().is_some());
    assert_eq!(requests[0].tenant().unwrap().as_str(), "acme-corp");
}
