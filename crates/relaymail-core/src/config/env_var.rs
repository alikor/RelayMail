use super::error::ConfigError;

/// Read a required env var; empty string is treated as unset.
pub fn read_required(name: &str) -> Result<String, ConfigError> {
    match std::env::var(name) {
        Ok(value) if !value.is_empty() => Ok(value),
        _ => Err(ConfigError::MissingVar(name.to_string())),
    }
}

/// Read an optional env var. Empty strings are treated as unset.
pub fn read_optional(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.is_empty())
}

pub fn read_bool(name: &str, default: bool) -> Result<bool, ConfigError> {
    match read_optional(name) {
        None => Ok(default),
        Some(v) => match v.to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Ok(true),
            "0" | "false" | "no" | "off" => Ok(false),
            other => Err(ConfigError::invalid(name, format!("bool `{other}`"))),
        },
    }
}

pub fn read_u32(name: &str, default: u32) -> Result<u32, ConfigError> {
    match read_optional(name) {
        None => Ok(default),
        Some(v) => v
            .parse::<u32>()
            .map_err(|_| ConfigError::invalid(name, format!("u32 `{v}`"))),
    }
}

pub fn read_u64(name: &str, default: u64) -> Result<u64, ConfigError> {
    match read_optional(name) {
        None => Ok(default),
        Some(v) => v
            .parse::<u64>()
            .map_err(|_| ConfigError::invalid(name, format!("u64 `{v}`"))),
    }
}

/// Parse a comma-separated list, trimming whitespace and dropping empties.
pub fn parse_csv_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_list_trims_and_skips_empty() {
        assert_eq!(parse_csv_list(" a , b ,, c "), vec!["a", "b", "c"]);
        assert!(parse_csv_list("").is_empty());
    }
}
