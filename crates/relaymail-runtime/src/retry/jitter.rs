use std::time::{SystemTime, UNIX_EPOCH};

/// Jitter factor in `[0.75, 1.25]` — trades back and forth deterministically
/// enough for tests when the clock is mocked, and diverse enough for real
/// retry fan-out in production.
pub(crate) fn factor() -> f64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    let normalized = (nanos as f64) / 1_000_000_000.0;
    0.75 + normalized * 0.5
}
