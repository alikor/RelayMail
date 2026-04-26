//! Configuration loading for `relaymail-email-ses`.

pub(crate) mod aws;
pub(crate) mod delivery;
pub(crate) mod error;
pub(crate) mod flat;
pub(crate) mod general;
pub(crate) mod loader;
pub(crate) mod polling;
pub(crate) mod processing;
pub(crate) mod runtime;
pub(crate) mod s3_filter;
pub(crate) mod ses;
pub(crate) mod sources;
pub(crate) mod sqs;

pub(crate) use self::loader::{load, AppConfig};
