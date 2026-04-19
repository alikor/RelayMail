use relaymail_core::{IdempotencyKey, TenantId};
use relaymail_delivery::{EmailSender, SendRequest, SendResult};
use relaymail_email::{EmailMetadata, RawEmail};

use super::error::StageError;

pub(crate) async fn send(
    sender: &dyn EmailSender,
    raw: RawEmail,
    meta: EmailMetadata,
    key: IdempotencyKey,
    tenant: Option<TenantId>,
) -> Result<SendResult, StageError> {
    let mut request = SendRequest::new(raw, meta, key);
    if let Some(t) = tenant {
        request = request.with_tenant(t);
    }
    Ok(sender.send(request).await?)
}
