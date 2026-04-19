//! HTTP server for health, readiness, and Prometheus metrics endpoints.

pub(crate) mod error;
pub(crate) mod handlers;
pub(crate) mod readiness;
pub(crate) mod router;
pub(crate) mod server;

pub use self::error::HttpServerError;
pub use self::readiness::ReadinessTracker;
pub use self::router::build_router;
pub use self::server::{serve, HealthServer};
