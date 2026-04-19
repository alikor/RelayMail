use aws_smithy_runtime_api::client::result::SdkError;
use relaymail_core::object_store::ObjectStoreError;

/// Map a generic SDK error into the domain-level [`ObjectStoreError`].
pub(crate) fn map_sdk_error<E, R>(err: SdkError<E, R>, fallback: &str) -> ObjectStoreError
where
    E: std::fmt::Debug,
    R: std::fmt::Debug,
{
    match err {
        SdkError::TimeoutError(_) => ObjectStoreError::Transient(fallback.to_string()),
        SdkError::DispatchFailure(_) => ObjectStoreError::Transient(fallback.to_string()),
        SdkError::ResponseError(_) => ObjectStoreError::Transient(fallback.to_string()),
        SdkError::ServiceError(e) => ObjectStoreError::Permanent(format!("{fallback}: {e:?}")),
        SdkError::ConstructionFailure(_) => ObjectStoreError::Permanent(fallback.to_string()),
        _ => ObjectStoreError::Permanent(fallback.to_string()),
    }
}
