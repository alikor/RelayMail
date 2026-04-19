use std::time::Duration;

/// Settings for the optional polling fallback.
///
/// When `enabled == false` (the default), the polling loop must not run.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PollingConfig {
    pub enabled: bool,
    pub interval: Duration,
    pub buckets: Vec<String>,
    pub prefixes: Vec<String>,
}

impl PollingConfig {
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            interval: Duration::from_secs(60),
            buckets: vec![],
            prefixes: vec![],
        }
    }
}

/// String the binary should log at startup when falling back to polling.
pub fn polling_disabled_warning() -> &'static str {
    "RelayMail polling mode disabled; using SQS event ingestion only"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_default() {
        let c = PollingConfig::disabled();
        assert!(!c.enabled);
        assert_eq!(c.interval, Duration::from_secs(60));
    }
}
