use serde::{Deserialize, Serialize};

/// Identity of a source object (typically an S3 object) being processed.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ObjectId {
    bucket: String,
    key: String,
    version_id: Option<String>,
}

impl ObjectId {
    pub fn new(bucket: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
            version_id: None,
        }
    }

    pub fn with_version(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }

    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn version_id(&self) -> Option<&str> {
        self.version_id.as_deref()
    }

    /// Return the extension of the key, lowercased, including the dot.
    ///
    /// For `incoming/msg.EML` this returns `Some(".eml")`. Returns `None` if
    /// the key has no dot-segment or the name is empty after the dot.
    pub fn extension(&self) -> Option<String> {
        let file = self.key.rsplit('/').next().unwrap_or(&self.key);
        let dot = file.rfind('.')?;
        let ext = &file[dot..];
        if ext.len() == 1 {
            None
        } else {
            Some(ext.to_ascii_lowercase())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_and_accesses_fields() {
        let id = ObjectId::new("b", "k/p.eml").with_version("v1");
        assert_eq!(id.bucket(), "b");
        assert_eq!(id.key(), "k/p.eml");
        assert_eq!(id.version_id(), Some("v1"));
    }

    #[test]
    fn extension_lowercases_and_strips_path() {
        assert_eq!(
            ObjectId::new("b", "a/b/C.EML").extension().as_deref(),
            Some(".eml")
        );
        assert_eq!(
            ObjectId::new("b", "flat.emi").extension().as_deref(),
            Some(".emi")
        );
    }

    #[test]
    fn extension_none_when_missing() {
        assert_eq!(ObjectId::new("b", "nodot").extension(), None);
        assert_eq!(ObjectId::new("b", "trailing.").extension(), None);
    }
}
