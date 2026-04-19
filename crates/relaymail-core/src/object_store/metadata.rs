use chrono::{DateTime, Utc};

/// Metadata returned for an object fetch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectMetadata {
    etag: String,
    size: u64,
    last_modified: Option<DateTime<Utc>>,
    content_type: Option<String>,
}

impl ObjectMetadata {
    pub fn new(etag: impl Into<String>, size: u64) -> Self {
        Self {
            etag: etag.into(),
            size,
            last_modified: None,
            content_type: None,
        }
    }

    pub fn with_last_modified(mut self, at: DateTime<Utc>) -> Self {
        self.last_modified = Some(at);
        self
    }

    pub fn with_content_type(mut self, ct: impl Into<String>) -> Self {
        self.content_type = Some(ct.into());
        self
    }

    pub fn etag(&self) -> &str {
        &self.etag
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn last_modified(&self) -> Option<DateTime<Utc>> {
        self.last_modified
    }

    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn getters_with_optional_fields() {
        let now = Utc::now();
        let m = ObjectMetadata::new("etag-1", 42)
            .with_last_modified(now)
            .with_content_type("message/rfc822");
        assert_eq!(m.etag(), "etag-1");
        assert_eq!(m.size(), 42);
        assert_eq!(m.last_modified(), Some(now));
        assert_eq!(m.content_type(), Some("message/rfc822"));
    }

    #[test]
    fn defaults_are_none() {
        let m = ObjectMetadata::new("e", 0);
        assert!(m.last_modified().is_none());
        assert!(m.content_type().is_none());
    }
}
