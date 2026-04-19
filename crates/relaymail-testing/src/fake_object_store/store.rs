use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use bytes::Bytes;
use relaymail_core::object_store::{FetchedObject, ObjectStore, ObjectStoreError, TagSet};
use relaymail_core::{ObjectId, ObjectMetadata};

use super::tag_recorder::TagRecord;

/// Process-local fake object store. Not for production.
#[derive(Debug, Default)]
pub struct FakeObjectStore {
    objects: Mutex<HashMap<String, (Bytes, ObjectMetadata)>>,
    tagging: Mutex<Vec<TagRecord>>,
    moves: Mutex<Vec<(ObjectId, String)>>,
    deletes: Mutex<Vec<ObjectId>>,
}

fn key(id: &ObjectId) -> String {
    format!("{}::{}", id.bucket(), id.key())
}

impl FakeObjectStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn put(&self, id: ObjectId, bytes: Bytes, metadata: ObjectMetadata) {
        self.objects
            .lock()
            .expect("poisoned")
            .insert(key(&id), (bytes, metadata));
    }

    pub fn tag_records(&self) -> Vec<TagRecord> {
        self.tagging.lock().expect("poisoned").clone()
    }

    pub fn moves(&self) -> Vec<(ObjectId, String)> {
        self.moves.lock().expect("poisoned").clone()
    }

    pub fn deletes(&self) -> Vec<ObjectId> {
        self.deletes.lock().expect("poisoned").clone()
    }
}

#[async_trait]
impl ObjectStore for FakeObjectStore {
    async fn fetch(&self, id: &ObjectId, max_size: u64) -> Result<FetchedObject, ObjectStoreError> {
        let guard = self.objects.lock().expect("poisoned");
        let entry = guard
            .get(&key(id))
            .ok_or_else(|| ObjectStoreError::NotFound(id.key().to_string()))?;
        if entry.1.size() > max_size {
            return Err(ObjectStoreError::TooLarge {
                actual: entry.1.size(),
                limit: max_size,
            });
        }
        Ok(FetchedObject {
            bytes: entry.0.clone(),
            metadata: entry.1.clone(),
        })
    }

    async fn tag(&self, id: &ObjectId, tags: &TagSet) -> Result<(), ObjectStoreError> {
        self.tagging
            .lock()
            .expect("poisoned")
            .push(TagRecord::new(id.clone(), tags.clone()));
        Ok(())
    }

    async fn move_to(&self, id: &ObjectId, dest_key: &str) -> Result<(), ObjectStoreError> {
        self.moves
            .lock()
            .expect("poisoned")
            .push((id.clone(), dest_key.to_string()));
        Ok(())
    }

    async fn delete(&self, id: &ObjectId) -> Result<(), ObjectStoreError> {
        self.deletes.lock().expect("poisoned").push(id.clone());
        Ok(())
    }
}
