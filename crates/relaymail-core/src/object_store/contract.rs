use async_trait::async_trait;
use bytes::Bytes;

use crate::ids::ObjectId;

use super::error::ObjectStoreError;
use super::metadata::ObjectMetadata;

/// Body + metadata for a fetched object.
#[derive(Clone, Debug)]
pub struct FetchedObject {
    pub bytes: Bytes,
    pub metadata: ObjectMetadata,
}

/// Tags applied to the source object post-processing. Key/value pairs follow
/// the provider's limits (S3: 10 tags, 128-char keys, 256-char values).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TagSet(Vec<(String, String)>);

impl TagSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.push((key.into(), value.into()));
    }

    pub fn entries(&self) -> &[(String, String)] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Capability trait for fetching and dispositioning source objects.
#[async_trait]
pub trait ObjectStore: Send + Sync + std::fmt::Debug {
    async fn fetch(&self, id: &ObjectId, max_size: u64) -> Result<FetchedObject, ObjectStoreError>;
    async fn tag(&self, id: &ObjectId, tags: &TagSet) -> Result<(), ObjectStoreError>;
    async fn move_to(&self, id: &ObjectId, dest_key: &str) -> Result<(), ObjectStoreError>;
    async fn delete(&self, id: &ObjectId) -> Result<(), ObjectStoreError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_set_accumulates_pairs() {
        let mut t = TagSet::new();
        t.insert("k", "v");
        assert_eq!(t.entries()[0], ("k".into(), "v".into()));
        assert_eq!(t.len(), 1);
        assert!(!t.is_empty());
    }
}
