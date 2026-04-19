//! Bounded worker pool with graceful drain.

pub(crate) mod outcome;
pub(crate) mod pool;

pub use self::outcome::JobOutcome;
pub use self::pool::WorkerPool;
