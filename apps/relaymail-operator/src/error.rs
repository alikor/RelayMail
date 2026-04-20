use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Kubernetes API error: {0}")]
    Kube(#[from] kube::Error),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("{0} has no namespace")]
    MissingNamespace(&'static str),
    #[error("{0} has no UID")]
    MissingUid(&'static str),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
