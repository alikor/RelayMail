/// Outcome reported by a processing job.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JobOutcome {
    Success,
    Retry,
    DeadLetter,
    Dropped,
}
