use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::ids::{ObjectId, TenantId};

/// Idempotency key: hex-encoded SHA-256 of
/// `tenant | bucket | key | version_or_etag | size`.
///
/// The key is stable across retries of the same source object, and changes
/// when the object is replaced (because its etag or size changes).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    pub fn compute(
        tenant: Option<&TenantId>,
        object: &ObjectId,
        version_or_etag: &str,
        size_bytes: u64,
    ) -> Self {
        let tenant_part = tenant.map(TenantId::as_str).unwrap_or("");
        let mut hasher = Sha256::new();
        for chunk in [
            tenant_part,
            "|",
            object.bucket(),
            "|",
            object.key(),
            "|",
            version_or_etag,
            "|",
        ] {
            hasher.update(chunk.as_bytes());
        }
        hasher.update(size_bytes.to_string().as_bytes());
        let digest = hasher.finalize();
        Self(hex(&digest))
    }

    pub fn from_hex(raw: impl Into<String>) -> Self {
        Self(raw.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// First 16 hex chars — suitable for log-safe display without revealing
    /// the full key.
    pub fn short(&self) -> &str {
        &self.0[..self.0.len().min(16)]
    }
}

fn hex(bytes: &[u8]) -> String {
    const CHARS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(CHARS[(b >> 4) as usize] as char);
        out.push(CHARS[(b & 0x0f) as usize] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_for_same_inputs() {
        let t = TenantId::parse("acme").unwrap();
        let o = ObjectId::new("b", "k.eml");
        let a = IdempotencyKey::compute(Some(&t), &o, "etag1", 42);
        let b = IdempotencyKey::compute(Some(&t), &o, "etag1", 42);
        assert_eq!(a, b);
        assert_eq!(a.as_str().len(), 64);
        assert_eq!(a.short().len(), 16);
    }

    #[test]
    fn diverges_on_any_input_change() {
        let t = TenantId::parse("acme").unwrap();
        let o = ObjectId::new("b", "k.eml");
        let base = IdempotencyKey::compute(Some(&t), &o, "etag1", 42);
        let diff_etag = IdempotencyKey::compute(Some(&t), &o, "etag2", 42);
        let diff_size = IdempotencyKey::compute(Some(&t), &o, "etag1", 43);
        assert_ne!(base, diff_etag);
        assert_ne!(base, diff_size);
    }
}
