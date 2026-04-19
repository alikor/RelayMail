use bytes::Bytes;

/// Owned raw MIME bytes of an email.
///
/// The bytes are treated as opaque — we don't rewrite or reorder them so
/// signatures (DKIM) and attachments stay intact.
#[derive(Clone, Debug)]
pub struct RawEmail {
    bytes: Bytes,
}

impl RawEmail {
    pub fn new(bytes: Bytes) -> Self {
        Self { bytes }
    }

    pub fn from_slice(bytes: &[u8]) -> Self {
        Self {
            bytes: Bytes::copy_from_slice(bytes),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn into_bytes(self) -> Bytes {
        self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_and_exposes_bytes() {
        let e = RawEmail::from_slice(b"hello");
        assert_eq!(e.len(), 5);
        assert!(!e.is_empty());
        assert_eq!(e.as_bytes(), b"hello");
    }
}
