use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use metrics_exporter_prometheus::PrometheusHandle;

use super::handlers::{healthz, metrics_endpoint, readyz, AppState};
use super::readiness::ReadinessTracker;
use crate::webhooks::{postmark_webhook, resend_webhook, smtp2go_webhook, WebhookState};

/// Build the axum router serving `/healthz`, `/readyz`, and `/metrics`.
pub fn build_router(readiness: Arc<ReadinessTracker>, prometheus: Arc<PrometheusHandle>) -> Router {
    build_router_with_webhooks(readiness, prometheus, None)
}

pub fn build_router_with_webhooks(
    readiness: Arc<ReadinessTracker>,
    prometheus: Arc<PrometheusHandle>,
    webhooks: Option<Arc<WebhookState>>,
) -> Router {
    let state = AppState {
        readiness,
        prometheus,
        webhooks,
    };
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/metrics", get(metrics_endpoint))
        .route("/api/relaymail/webhooks/resend", post(resend_webhook))
        .route("/api/relaymail/webhooks/postmark", post(postmark_webhook))
        .route("/api/relaymail/webhooks/smtp2go", post(smtp2go_webhook))
        .with_state(state)
}
