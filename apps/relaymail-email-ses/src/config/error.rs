use relaymail_core::config::ConfigError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum AppConfigError {
    #[error("missing required env var: {0}")]
    Missing(String),

    #[error("invalid value: {0}")]
    Invalid(String),

    #[error("figment error: {0}")]
    Figment(String),
}

impl From<ConfigError> for AppConfigError {
    fn from(err: ConfigError) -> Self {
        match err {
            ConfigError::MissingVar(name) => Self::Missing(name),
            ConfigError::InvalidValue { name, reason } => {
                Self::Invalid(format!("{name}: {reason}"))
            }
        }
    }
}

impl From<figment::Error> for AppConfigError {
    fn from(err: figment::Error) -> Self {
        Self::Figment(err.to_string())
    }
}

impl From<Box<figment::Error>> for AppConfigError {
    fn from(err: Box<figment::Error>) -> Self {
        Self::Figment(err.to_string())
    }
}
