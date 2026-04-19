use std::time::Duration;

use super::error::ConfigError;

/// Parse a human-friendly duration into seconds.
///
/// Accepts `"30"`, `"30s"`, `"2m"`, `"1h"`. No fractional values.
pub fn parse_duration_seconds(name: &str, raw: &str) -> Result<Duration, ConfigError> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Err(ConfigError::invalid(name, "empty duration"));
    }
    let (digits, unit_secs) = split_suffix(raw);
    let value: u64 = digits
        .parse()
        .map_err(|_| ConfigError::invalid(name, format!("duration `{raw}`")))?;
    let seconds = value
        .checked_mul(unit_secs)
        .ok_or_else(|| ConfigError::invalid(name, "duration overflow"))?;
    Ok(Duration::from_secs(seconds))
}

fn split_suffix(raw: &str) -> (&str, u64) {
    if let Some(body) = raw.strip_suffix('s') {
        (body, 1)
    } else if let Some(body) = raw.strip_suffix('m') {
        (body, 60)
    } else if let Some(body) = raw.strip_suffix('h') {
        (body, 3600)
    } else {
        (raw, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_units() {
        assert_eq!(
            parse_duration_seconds("n", "30").unwrap(),
            Duration::from_secs(30)
        );
        assert_eq!(
            parse_duration_seconds("n", "30s").unwrap(),
            Duration::from_secs(30)
        );
        assert_eq!(
            parse_duration_seconds("n", "2m").unwrap(),
            Duration::from_secs(120)
        );
        assert_eq!(
            parse_duration_seconds("n", "1h").unwrap(),
            Duration::from_secs(3600)
        );
    }

    #[test]
    fn rejects_garbage() {
        assert!(parse_duration_seconds("n", "").is_err());
        assert!(parse_duration_seconds("n", "abc").is_err());
        assert!(parse_duration_seconds("n", "5x").is_err());
    }
}
