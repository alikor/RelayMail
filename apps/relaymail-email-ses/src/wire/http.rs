use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context;
use relaymail_runtime::{
    build_router, init_prometheus_handle, serve, HealthServer, ReadinessTracker, ShutdownToken,
};

use crate::config::AppConfig;

pub(crate) async fn start_http_server(
    cfg: &AppConfig,
    readiness: Arc<ReadinessTracker>,
    shutdown: ShutdownToken,
) -> anyhow::Result<HealthServer> {
    let addr: SocketAddr = cfg
        .runtime
        .http_bind_addr
        .parse()
        .with_context(|| format!("parsing http bind addr `{}`", cfg.runtime.http_bind_addr))?;
    let handle = init_prometheus_handle().map_err(|e| anyhow::anyhow!("init prometheus: {e}"))?;
    let router = build_router(readiness, handle.handle());
    let server = serve(addr, router, shutdown).await?;
    Ok(server)
}
