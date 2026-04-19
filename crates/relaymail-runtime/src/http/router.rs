use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use metrics_exporter_prometheus::PrometheusHandle;

use super::handlers::{healthz, metrics_endpoint, readyz, AppState};
use super::readiness::ReadinessTracker;

/// Build the axum router serving `/healthz`, `/readyz`, and `/metrics`.
pub fn build_router(readiness: Arc<ReadinessTracker>, prometheus: Arc<PrometheusHandle>) -> Router {
    let state = AppState {
        readiness,
        prometheus,
    };
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/metrics", get(metrics_endpoint))
        .with_state(state)
}
