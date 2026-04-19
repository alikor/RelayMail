/// HTTP server errors.
#[derive(Debug, thiserror::Error)]
pub enum HttpServerError {
    #[error("failed to bind address: {0}")]
    Bind(#[source] std::io::Error),

    #[error("server error: {0}")]
    Serve(#[source] std::io::Error),
}
