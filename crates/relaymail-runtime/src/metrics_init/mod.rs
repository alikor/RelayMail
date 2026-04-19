//! Prometheus-backed metrics registration.

pub(crate) mod descriptors;
pub(crate) mod emit;
pub(crate) mod registry;

pub use self::emit::{emit_processed, emit_send_latency};
pub use self::registry::{init_prometheus_handle, MetricsHandle};
