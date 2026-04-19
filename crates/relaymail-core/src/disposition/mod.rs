//! Disposition policy — maps error classification + attempt into a decision.

pub(crate) mod attempt;
pub(crate) mod classification;
pub(crate) mod policy;

pub use self::attempt::AttemptCount;
pub use self::classification::ErrorClassification;
pub use self::policy::{DispositionDecision, DispositionPolicy, RetryLimits};
