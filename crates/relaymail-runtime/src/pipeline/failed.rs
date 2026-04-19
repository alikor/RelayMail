use super::ctx::PipelineCtx;
use super::error::StageError;
use super::event_parser::ObjectRef;
use super::outcome::PipelineOutcome;
use super::stage_dispose;
use crate::metrics_init::emit::emit_failure;

/// Emit failure metrics, apply failure-disposition if classification is
/// permanent, and return the `Failed` outcome.
pub(crate) async fn failed(
    ctx: &PipelineCtx,
    object: &ObjectRef,
    err: &StageError,
) -> PipelineOutcome {
    let class = err.classification();
    emit_failure(&ctx.config.service_name, class.label());
    if class.is_permanent() {
        let ts = ctx.clock.now().to_rfc3339();
        let _ = stage_dispose::on_failure(
            ctx.object_store.as_ref(),
            &ctx.config,
            &object.object,
            class.label(),
            &ts,
        )
        .await;
    }
    PipelineOutcome::Failed {
        classification_label: class.label(),
        reason: err.to_string(),
    }
}
