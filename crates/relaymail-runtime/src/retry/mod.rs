//! Retry timing (the "when" of retry; the "whether" lives in the core
//! disposition policy).

pub(crate) mod jitter;
pub(crate) mod policy;

pub use self::policy::RetryPolicy;
