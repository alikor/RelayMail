use relaymail_core::IdempotencyKey;
use relaymail_delivery::SendResult;

use super::ctx::PipelineCtx;
use super::event_parser::ObjectRef;
use super::outcome::PipelineOutcome;
use super::stage_dispose;
use crate::metrics_init::emit::{emit_processed, emit_sent};

/// Run the success-side disposition and return the `Sent` outcome.
pub(crate) async fn on_send_success(
    ctx: &PipelineCtx,
    object: &ObjectRef,
    key: &IdempotencyKey,
    result: SendResult,
) -> PipelineOutcome {
    let _ = ctx
        .idempotency_store
        .mark_sent(key, result.provider_message_id())
        .await;
    let _ = stage_dispose::on_success(
        ctx.object_store.as_ref(),
        &ctx.config,
        &object.object,
        result.provider_message_id(),
        &result.accepted_at().to_rfc3339(),
    )
    .await;
    emit_sent(&ctx.config.service_name, &ctx.config.provider_label);
    emit_processed(&ctx.config.service_name, &ctx.config.provider_label, "sent");
    PipelineOutcome::Sent {
        provider_message_id: result.provider_message_id().into(),
    }
}
