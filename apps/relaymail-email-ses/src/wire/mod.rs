//! Dependency-wiring from `AppConfig` into runtime components.

pub(crate) mod aws;
pub(crate) mod delivery;
pub(crate) mod domain;
pub(crate) mod http;

pub(crate) use self::aws::build_aws_clients;
pub(crate) use self::delivery::build_delivery;
pub(crate) use self::domain::build_pipeline;
pub(crate) use self::http::start_http_server;
