//! Optional polling fallback (S3 `ListObjectsV2`-style), disabled by default.

pub(crate) mod loop_driver;

pub use self::loop_driver::{polling_disabled_warning, PollingConfig};
