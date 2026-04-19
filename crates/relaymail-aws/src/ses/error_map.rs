use aws_sdk_sesv2::operation::send_email::SendEmailError;
use aws_smithy_runtime_api::client::result::SdkError;
use relaymail_delivery::SendError;

/// Classify an SES v2 SDK error into the provider-agnostic [`SendError`].
pub(crate) fn map_sdk_error<R>(err: SdkError<SendEmailError, R>) -> SendError
where
    R: std::fmt::Debug,
{
    match err {
        SdkError::TimeoutError(_) | SdkError::DispatchFailure(_) | SdkError::ResponseError(_) => {
            SendError::Transient(format!("{err:?}"))
        }
        SdkError::ServiceError(e) => map_service(e.into_err()),
        SdkError::ConstructionFailure(_) => SendError::Validation(format!("{err:?}")),
        _ => SendError::Transient(format!("{err:?}")),
    }
}

fn map_service(err: SendEmailError) -> SendError {
    use SendEmailError as E;
    let message = format!("{err:?}");
    match err {
        E::TooManyRequestsException(_) => SendError::Throttled(message),
        E::LimitExceededException(_) => SendError::QuotaExceeded(message),
        E::SendingPausedException(_) => SendError::Permanent(message),
        E::AccountSuspendedException(_) => SendError::Permanent(message),
        E::MailFromDomainNotVerifiedException(_) => SendError::AuthenticationFailure(message),
        E::MessageRejected(_) => SendError::Validation(message),
        E::BadRequestException(_) => SendError::Validation(message),
        E::NotFoundException(_) => SendError::AuthenticationFailure(message),
        _ => SendError::Permanent(message),
    }
}
