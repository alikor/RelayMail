use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use metrics_exporter_prometheus::PrometheusHandle;

use super::readiness::ReadinessTracker;

use crate::webhooks::WebhookState;

#[derive(Clone)]
pub(crate) struct AppState {
    pub readiness: Arc<ReadinessTracker>,
    pub prometheus: Arc<PrometheusHandle>,
    pub webhooks: Option<Arc<WebhookState>>,
}

pub(crate) async fn healthz() -> impl IntoResponse {
    (StatusCode::OK, "ok\n")
}

pub(crate) async fn readyz(State(s): State<AppState>) -> impl IntoResponse {
    if s.readiness.is_ready() {
        (StatusCode::OK, "ready\n").into_response()
    } else {
        let body = s
            .readiness
            .snapshot()
            .into_iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(",");
        (
            StatusCode::SERVICE_UNAVAILABLE,
            format!("not ready: {body}\n"),
        )
            .into_response()
    }
}

pub(crate) async fn metrics_endpoint(State(s): State<AppState>) -> impl IntoResponse {
    let body = s.prometheus.render();
    ([("content-type", "text/plain; version=0.0.4")], body)
}
