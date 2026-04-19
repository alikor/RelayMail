use relaymail_core::RawEnvelope;

use super::ctx::PipelineCtx;
use super::event_parser::ObjectRef;
use super::failed::failed;
use super::outcome::PipelineOutcome;
use super::stage_claim::ClaimDecision;
use super::success::on_send_success;
use super::{stage_claim, stage_fetch, stage_filter, stage_send, stage_validate};
use crate::metrics_init::emit::{emit_idempotency_skip, emit_processed, emit_processing_duration};
use crate::metrics_init::emit_send_latency;

/// Process one envelope. Returns one outcome per object the envelope yields.
pub async fn process_envelope(ctx: &PipelineCtx, envelope: &RawEnvelope) -> Vec<PipelineOutcome> {
    let refs = match ctx.event_parser.parse(envelope.body()) {
        Ok(v) => v,
        Err(_) => return vec![PipelineOutcome::UnknownEnvelope],
    };
    let mut out = Vec::with_capacity(refs.len());
    for object in refs {
        out.push(process_one(ctx, &object).await);
    }
    out
}

pub(crate) async fn process_one(ctx: &PipelineCtx, object: &ObjectRef) -> PipelineOutcome {
    let started = ctx.clock.now();
    match stage_filter::filter(&ctx.config, object) {
        stage_filter::FilterDecision::Skip(skip) => {
            emit_processed(
                &ctx.config.service_name,
                &ctx.config.provider_label,
                "skipped",
            );
            return skip;
        }
        stage_filter::FilterDecision::Pass => {}
    }
    let key = stage_claim::compute_key(ctx.tenant_id.as_ref(), object);
    match stage_claim::claim(
        ctx.idempotency_store.as_ref(),
        &key,
        ctx.clock.now(),
        ctx.config.idempotency_ttl_seconds,
    )
    .await
    {
        Ok(ClaimDecision::AlreadySent) | Ok(ClaimDecision::AlreadyClaimed) => {
            emit_idempotency_skip(&ctx.config.service_name);
            return PipelineOutcome::SkippedAlreadyClaimed;
        }
        Ok(ClaimDecision::Proceed) => {}
        Err(e) => return failed(ctx, object, &e).await,
    }
    let fetched = match stage_fetch::fetch(
        ctx.object_store.as_ref(),
        object,
        ctx.config.max_object_size_bytes,
    )
    .await
    {
        Ok(f) => f,
        Err(e) => return failed(ctx, object, &e).await,
    };
    let (raw, meta) =
        match stage_validate::to_raw_and_metadata(fetched, ctx.config.max_object_size_bytes) {
            Ok(p) => p,
            Err(e) => return failed(ctx, object, &e).await,
        };
    if ctx.config.dry_run {
        let _ = ctx.idempotency_store.mark_sent(&key, "dry-run").await;
        emit_processed(
            &ctx.config.service_name,
            &ctx.config.provider_label,
            "dry_run",
        );
        return PipelineOutcome::DryRunSent;
    }
    let send_start = ctx.clock.now();
    let outcome = match stage_send::send(
        ctx.email_sender.as_ref(),
        raw,
        meta,
        key.clone(),
        ctx.tenant_id.clone(),
    )
    .await
    {
        Ok(result) => {
            let elapsed = (ctx.clock.now() - send_start).num_milliseconds() as f64 / 1000.0;
            emit_send_latency(&ctx.config.service_name, elapsed.max(0.0));
            on_send_success(ctx, object, &key, result).await
        }
        Err(e) => failed(ctx, object, &e).await,
    };
    let total = (ctx.clock.now() - started).num_milliseconds() as f64 / 1000.0;
    emit_processing_duration(&ctx.config.service_name, total.max(0.0));
    outcome
}
