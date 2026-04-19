/// Errors raised by the configuration helpers.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ConfigError {
    #[error("required env var `{0}` is not set")]
    MissingVar(String),

    #[error("env var `{name}` has invalid value: {reason}")]
    InvalidValue { name: String, reason: String },
}

impl ConfigError {
    pub fn invalid(name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidValue {
            name: name.into(),
            reason: reason.into(),
        }
    }
}
