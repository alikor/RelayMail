use super::error::AppConfigError;
use super::flat::FlatConfig;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(crate) struct RuntimeConfig {
    pub worker_concurrency: usize,
    pub http_bind_addr: String,
    pub shutdown_grace_period_seconds: u64,
    pub idempotency_table_name: Option<String>,
    pub idempotency_ttl_seconds: u64,
    pub transport_state_table_name: Option<String>,
}

impl RuntimeConfig {
    pub(crate) fn from_flat(flat: &FlatConfig) -> Result<Self, AppConfigError> {
        let addr = flat
            .http_bind_addr
            .clone()
            .unwrap_or_else(|| "0.0.0.0:8080".to_string());
        if addr.is_empty() {
            return Err(AppConfigError::Invalid("http_bind_addr empty".into()));
        }
        Ok(Self {
            worker_concurrency: flat.worker_concurrency.unwrap_or(4).max(1),
            http_bind_addr: addr,
            shutdown_grace_period_seconds: flat.shutdown_grace_period_seconds.unwrap_or(30),
            idempotency_table_name: flat
                .idempotency_table_name
                .clone()
                .filter(|s| !s.is_empty()),
            idempotency_ttl_seconds: flat.idempotency_ttl_seconds.unwrap_or(604800),
            transport_state_table_name: flat
                .transport_state_table_name
                .clone()
                .filter(|s| !s.is_empty()),
        })
    }
}
