use std::sync::Arc;

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use relaymail_runtime::{build_router, init_prometheus_handle, ReadinessTracker};
use tower::ServiceExt;

fn router_with_components() -> axum::Router {
    let readiness = Arc::new(ReadinessTracker::new());
    readiness.register("pipeline");
    let handle = init_prometheus_handle()
        .ok()
        .map(|h| h.handle())
        .unwrap_or_else(|| {
            Arc::new(
                metrics_exporter_prometheus::PrometheusBuilder::new()
                    .build_recorder()
                    .handle(),
            )
        });
    build_router(readiness.clone(), handle)
}

#[tokio::test]
async fn healthz_returns_ok() {
    let response = router_with_components()
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(String::new())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), 128).await.unwrap();
    assert_eq!(body, &b"ok\n"[..]);
}

#[tokio::test]
async fn readyz_is_unavailable_until_ready() {
    let router = router_with_components();
    let r = router
        .oneshot(
            Request::builder()
                .uri("/readyz")
                .body(String::new())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(r.status(), StatusCode::SERVICE_UNAVAILABLE);
}
