use aws_smithy_runtime_api::client::result::SdkError;
use relaymail_core::message_source::MessageSourceError;

pub(crate) fn map_sdk_error<E, R>(err: SdkError<E, R>, fallback: &str) -> MessageSourceError
where
    E: std::fmt::Debug,
    R: std::fmt::Debug,
{
    match err {
        SdkError::TimeoutError(_) => MessageSourceError::Transient(fallback.to_string()),
        SdkError::DispatchFailure(_) => MessageSourceError::Transient(fallback.to_string()),
        SdkError::ResponseError(_) => MessageSourceError::Transient(fallback.to_string()),
        SdkError::ServiceError(e) => MessageSourceError::Permanent(format!("{fallback}: {e:?}")),
        _ => MessageSourceError::Permanent(fallback.to_string()),
    }
}
