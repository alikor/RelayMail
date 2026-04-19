use std::sync::Arc;

use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};

use super::descriptors::describe_all;

/// Handle to the installed Prometheus recorder.
#[derive(Clone)]
pub struct MetricsHandle {
    inner: Arc<PrometheusHandle>,
}

impl MetricsHandle {
    pub fn handle(&self) -> Arc<PrometheusHandle> {
        self.inner.clone()
    }

    pub fn render(&self) -> String {
        self.inner.render()
    }
}

impl std::fmt::Debug for MetricsHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetricsHandle").finish_non_exhaustive()
    }
}

/// Install the Prometheus recorder and register all metric descriptors.
///
/// Returns an error if called more than once per process.
pub fn init_prometheus_handle() -> Result<MetricsHandle, String> {
    let recorder = PrometheusBuilder::new()
        .install_recorder()
        .map_err(|e| e.to_string())?;
    describe_all();
    Ok(MetricsHandle {
        inner: Arc::new(recorder),
    })
}
