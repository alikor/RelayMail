//! Runtime infrastructure for RelayMail services.
//!
//! Provides: HTTP health/ready/metrics endpoints, Prometheus metrics,
//! tracing/JSON logs, worker-pool concurrency control, graceful shutdown,
//! retry/backoff policy, and a provider-agnostic processing pipeline.

pub mod http;
pub mod metrics_init;
pub mod pipeline;
pub mod polling;
pub mod retry;
pub mod shutdown;
pub mod tracing_init;
pub mod transport;
pub mod webhooks;
pub mod worker;

pub use self::http::{
    build_router, build_router_with_webhooks, serve, HealthServer, HttpServerError,
    ReadinessTracker,
};
pub use self::metrics_init::{init_prometheus_handle, MetricsHandle};
pub use self::pipeline::{
    EventParseError, EventParser, FailureDispositionMode, ObjectRef, PipelineCtx, PipelineOutcome,
    ProcessingConfig, StageError, SuccessDispositionMode,
};
pub use self::retry::RetryPolicy;
pub use self::shutdown::ShutdownToken;
pub use self::tracing_init::install_tracing;
pub use self::transport::{
    normalize_email, DeliveryPolicy, EmailEventRecord, EventRecordStatus, InMemoryTransportStore,
    MessageLogRecord, RelayMailDeliveryService, SendAttemptRecord, StreamPolicy, SuppressionRecord,
    TransportStore, TransportStoreError,
};
pub use self::webhooks::{WebhookAuthConfig, WebhookConfig, WebhookState};
pub use self::worker::{JobOutcome, WorkerPool};
