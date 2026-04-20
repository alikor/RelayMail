//! Kubernetes operator for `RelayMailSes` custom resources.
mod controller;
mod crd;
mod error;

use std::sync::Arc;

use anyhow::Context as _;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "relaymail_operator=info,kube_runtime=info,warn".parse().unwrap()
            }),
        )
        .init();

    info!("starting relaymail-operator");

    let client = kube::Client::try_default()
        .await
        .context("failed to build Kubernetes client")?;

    let ctx = Arc::new(controller::Context::new(client));
    controller::run(ctx).await
}
