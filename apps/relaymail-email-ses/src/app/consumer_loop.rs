use std::sync::Arc;
use std::time::{Duration, Instant};

use relaymail_runtime::pipeline::{process_envelope, PipelineCtx};
use relaymail_runtime::ShutdownToken;
use tracing::{error, info, warn};

/// Emit a "consumer heartbeat" log line at least this often so that prolonged
/// silence in the logs is unambiguously diagnosable. With a 20s SQS long-poll
/// and an empty queue this fires roughly every 15 idle polls.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(300);

/// Drive the processing loop: long-poll the source, process each envelope,
/// and exit cleanly on shutdown.
pub(crate) async fn run_consumer(ctx: Arc<PipelineCtx>, shutdown: ShutdownToken) {
    info!(target: "relaymail_email_ses", "starting consumer loop");
    let mut last_heartbeat = Instant::now();
    let mut polls_since_heartbeat: u64 = 0;
    let mut messages_since_heartbeat: u64 = 0;
    loop {
        if shutdown.is_cancelled() {
            break;
        }
        let envelopes = match ctx.message_source.receive().await {
            Ok(v) => v,
            Err(e) => {
                error!(target: "relaymail_email_ses", error = %e, "receive failed");
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                continue;
            }
        };
        polls_since_heartbeat += 1;
        messages_since_heartbeat += envelopes.len() as u64;
        for envelope in envelopes {
            let outcomes = process_envelope(&ctx, &envelope).await;
            let permanent = outcomes.iter().all(|o| !matches!(
                o,
                relaymail_runtime::pipeline::PipelineOutcome::Failed { classification_label, .. }
                    if *classification_label == "transient"
            ));
            if permanent {
                if let Err(e) = ctx.message_source.ack(&envelope).await {
                    warn!(target: "relaymail_email_ses", error = %e, "ack failed");
                }
            } else if let Err(e) = ctx.message_source.nack(&envelope).await {
                warn!(target: "relaymail_email_ses", error = %e, "nack failed");
            }
        }
        if last_heartbeat.elapsed() >= HEARTBEAT_INTERVAL {
            info!(
                target: "relaymail_email_ses",
                polls = polls_since_heartbeat,
                messages = messages_since_heartbeat,
                "consumer heartbeat"
            );
            last_heartbeat = Instant::now();
            polls_since_heartbeat = 0;
            messages_since_heartbeat = 0;
        }
    }
    info!(target: "relaymail_email_ses", "consumer loop exited");
}
