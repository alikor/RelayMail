use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;

use super::error::HttpServerError;
use crate::shutdown::ShutdownToken;

/// Handle for a started HTTP server.
#[derive(Debug)]
pub struct HealthServer {
    local_addr: SocketAddr,
}

impl HealthServer {
    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }
}

/// Bind to `addr`, serve `router` until `shutdown` fires, then drain.
pub async fn serve(
    addr: SocketAddr,
    router: Router,
    shutdown: ShutdownToken,
) -> Result<HealthServer, HttpServerError> {
    let listener = TcpListener::bind(addr)
        .await
        .map_err(HttpServerError::Bind)?;
    let local_addr = listener.local_addr().map_err(HttpServerError::Bind)?;
    tokio::spawn(async move {
        let _ = axum::serve(listener, router)
            .with_graceful_shutdown(async move { shutdown.cancelled().await })
            .await;
    });
    Ok(HealthServer { local_addr })
}
