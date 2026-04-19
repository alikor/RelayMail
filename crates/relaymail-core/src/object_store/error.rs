/// Errors raised by object-store adapters.
#[derive(Debug, thiserror::Error)]
pub enum ObjectStoreError {
    #[error("object not found: {0}")]
    NotFound(String),

    #[error("object exceeds configured max size ({actual} > {limit})")]
    TooLarge { actual: u64, limit: u64 },

    #[error("permission denied: {0}")]
    PermissionDenied(String),

    #[error("transient transport error: {0}")]
    Transient(String),

    #[error("permanent transport error: {0}")]
    Permanent(String),
}
