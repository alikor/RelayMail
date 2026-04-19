use tracing_subscriber::filter::EnvFilter;

/// Resolve a log directive, preferring the caller-provided level, then the
/// `RUST_LOG` env var, then a sensible default of `info`.
pub(crate) fn filter_from(level: &str) -> EnvFilter {
    if let Ok(existing) = EnvFilter::try_from_default_env() {
        return existing;
    }
    let directive = if level.is_empty() { "info" } else { level };
    EnvFilter::try_new(directive).unwrap_or_else(|_| EnvFilter::new("info"))
}
