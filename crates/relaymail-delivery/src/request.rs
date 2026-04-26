use relaymail_core::{IdempotencyKey, TenantId};
use relaymail_email::{EmailMetadata, RawEmail};

use super::normalized::EmailSendRequest;

/// Everything a provider adapter needs to send one raw email.
#[derive(Clone, Debug)]
pub struct SendRequest {
    raw: RawEmail,
    metadata: EmailMetadata,
    tenant: Option<TenantId>,
    idempotency_key: IdempotencyKey,
    email: Option<EmailSendRequest>,
}

impl SendRequest {
    pub fn new(raw: RawEmail, metadata: EmailMetadata, idempotency_key: IdempotencyKey) -> Self {
        Self {
            raw,
            metadata,
            tenant: None,
            idempotency_key,
            email: None,
        }
    }

    pub fn with_tenant(mut self, tenant: TenantId) -> Self {
        self.tenant = Some(tenant);
        self
    }

    pub fn with_email(mut self, email: EmailSendRequest) -> Self {
        self.email = Some(email);
        self
    }

    pub fn raw(&self) -> &RawEmail {
        &self.raw
    }

    pub fn metadata(&self) -> &EmailMetadata {
        &self.metadata
    }

    pub fn tenant(&self) -> Option<&TenantId> {
        self.tenant.as_ref()
    }

    pub fn idempotency_key(&self) -> &IdempotencyKey {
        &self.idempotency_key
    }

    pub fn email(&self) -> Option<&EmailSendRequest> {
        self.email.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use relaymail_core::{IdempotencyKey, ObjectId, TenantId};
    use relaymail_email::{parse_headers_only, EmailMetadata, RawEmail};

    const RAW: &[u8] = b"From: a@b.com\r\nTo: c@d.com\r\n\r\n";

    fn sample() -> SendRequest {
        let raw = RawEmail::from_slice(RAW);
        let headers = parse_headers_only(RAW).unwrap();
        let meta = EmailMetadata::from_headers(&headers, RAW.len() as u64);
        let key = IdempotencyKey::compute(None, &ObjectId::new("b", "k"), "e", 1);
        SendRequest::new(raw, meta, key)
    }

    #[test]
    fn accessors_return_provided_values() {
        let r = sample();
        assert_eq!(r.raw().as_bytes(), RAW);
        assert_eq!(r.idempotency_key().as_str().len(), 64);
        assert!(r.tenant().is_none());
        assert_eq!(r.metadata().size_bytes(), RAW.len() as u64);
    }

    #[test]
    fn with_tenant_sets_tenant() {
        let t = TenantId::parse("acme").unwrap();
        let r = sample().with_tenant(t);
        assert_eq!(r.tenant().unwrap().as_str(), "acme");
    }
}
