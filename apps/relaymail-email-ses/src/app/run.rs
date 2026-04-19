use std::sync::Arc;

use relaymail_runtime::{install_tracing, ReadinessTracker};
use tracing::info;

use super::consumer_loop::run_consumer;
use crate::config::load;
use crate::dry_run::log_dry_run_notice;
use crate::wire::{build_aws_clients, build_pipeline, start_http_server};

pub async fn run() -> anyhow::Result<()> {
    let cfg = load(std::env::var("RELAYMAIL_CONFIG_FILE").ok().map(Into::into))?;
    install_tracing(&cfg.general.log_level, cfg.general.log_json);
    if cfg.general.dry_run {
        log_dry_run_notice(&cfg.general.service_name);
    }
    if cfg.runtime.idempotency_table_name.is_none() {
        tracing::warn!(
            target: "relaymail_email_ses",
            "RELAYMAIL_IDEMPOTENCY_TABLE_NAME unset: using in-memory store (UNSAFE for prod)"
        );
    }
    let shutdown = relaymail_runtime::shutdown::install_shutdown_handler()?;
    let readiness = Arc::new(ReadinessTracker::new());
    readiness.register("aws_clients");
    readiness.register("pipeline");
    let _server = start_http_server(&cfg, readiness.clone(), shutdown.clone()).await?;
    let clients = build_aws_clients(&cfg).await;
    readiness.mark_ready("aws_clients");
    let pipeline = Arc::new(build_pipeline(&cfg, &clients));
    readiness.mark_ready("pipeline");
    info!(
        target: "relaymail_email_ses",
        service = %cfg.general.service_name,
        environment = %cfg.general.environment,
        "service started"
    );
    run_consumer(pipeline, shutdown.clone()).await;
    Ok(())
}
