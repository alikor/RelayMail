/// Parser-level errors.
#[derive(Debug, thiserror::Error)]
pub enum S3EventParseError {
    #[error("unknown envelope shape")]
    UnknownEnvelope,

    #[error("missing field: {0}")]
    MissingField(&'static str),

    #[error("invalid json: {0}")]
    InvalidJson(String),
}

impl From<serde_json::Error> for S3EventParseError {
    fn from(err: serde_json::Error) -> Self {
        Self::InvalidJson(err.to_string())
    }
}
