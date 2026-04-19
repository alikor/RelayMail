use aws_sdk_dynamodb::operation::put_item::PutItemError;
use aws_smithy_runtime_api::client::result::SdkError;
use relaymail_core::idempotency::IdempotencyError;

pub(crate) fn map_put_item_err<R>(err: SdkError<PutItemError, R>) -> PutItemOutcome
where
    R: std::fmt::Debug,
{
    match err {
        SdkError::ServiceError(e) => match e.into_err() {
            PutItemError::ConditionalCheckFailedException(_) => PutItemOutcome::AlreadyClaimed,
            other => PutItemOutcome::Err(IdempotencyError::Permanent(format!("{other:?}"))),
        },
        SdkError::TimeoutError(_) | SdkError::DispatchFailure(_) | SdkError::ResponseError(_) => {
            PutItemOutcome::Err(IdempotencyError::Transient(format!("{err:?}")))
        }
        _ => PutItemOutcome::Err(IdempotencyError::Permanent(format!("{err:?}"))),
    }
}

pub(crate) enum PutItemOutcome {
    AlreadyClaimed,
    Err(IdempotencyError),
}
