use std::sync::Arc;

use relaymail_runtime::pipeline::{process_envelope, PipelineCtx};
use relaymail_runtime::ShutdownToken;
use tracing::{error, info, warn};

/// Drive the processing loop: long-poll the source, process each envelope,
/// and exit cleanly on shutdown.
pub(crate) async fn run_consumer(ctx: Arc<PipelineCtx>, shutdown: ShutdownToken) {
    info!(target: "relaymail_email_ses", "starting consumer loop");
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
    }
    info!(target: "relaymail_email_ses", "consumer loop exited");
}
