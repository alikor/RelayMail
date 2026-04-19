use std::time::Duration;

use super::jitter::factor;

/// Exponential backoff with jitter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetryPolicy {
    base_ms: u64,
    max_ms: u64,
    max_attempts: u32,
}

impl RetryPolicy {
    pub fn new(base_ms: u64, max_ms: u64, max_attempts: u32) -> Self {
        Self {
            base_ms,
            max_ms,
            max_attempts,
        }
    }

    pub fn max_attempts(&self) -> u32 {
        self.max_attempts
    }

    /// Delay to wait before attempt number `attempt` (1-indexed, so the first
    /// retry uses attempt=2).
    pub fn delay(&self, attempt: u32) -> Duration {
        let exp = attempt.saturating_sub(1).min(16);
        let base = self.base_ms.saturating_mul(1u64 << exp);
        let capped = base.min(self.max_ms);
        let jittered = (capped as f64 * factor()) as u64;
        Duration::from_millis(jittered)
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::new(250, 30_000, 5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delay_grows_within_caps() {
        let p = RetryPolicy::new(100, 10_000, 10);
        let a = p.delay(1).as_millis();
        let b = p.delay(3).as_millis();
        assert!(a > 0);
        assert!(b >= a);
        assert!(p.delay(30).as_millis() <= (10_000.0 * 1.25) as u128);
    }
}
