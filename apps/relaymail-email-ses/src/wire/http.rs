use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context;
use relaymail_runtime::{
    build_router_with_webhooks, init_prometheus_handle, serve, HealthServer, ReadinessTracker,
    ShutdownToken, TransportStore, WebhookState,
};

use crate::config::AppConfig;

pub(crate) async fn start_http_server(
    cfg: &AppConfig,
    readiness: Arc<ReadinessTracker>,
    shutdown: ShutdownToken,
    transport_store: Arc<dyn TransportStore>,
) -> anyhow::Result<HealthServer> {
    let addr: SocketAddr = cfg
        .runtime
        .http_bind_addr
        .parse()
        .with_context(|| format!("parsing http bind addr `{}`", cfg.runtime.http_bind_addr))?;
    let handle = init_prometheus_handle().map_err(|e| anyhow::anyhow!("init prometheus: {e}"))?;
    let router = build_router_with_webhooks(
        readiness,
        handle.handle(),
        Some(Arc::new(WebhookState {
            config: cfg.delivery.webhook.clone(),
            store: transport_store,
        })),
    );
    let server = serve(addr, router, shutdown).await?;
    Ok(server)
}
