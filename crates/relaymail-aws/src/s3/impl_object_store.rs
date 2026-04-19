use async_trait::async_trait;
use relaymail_core::object_store::{FetchedObject, ObjectStore, ObjectStoreError, TagSet};
use relaymail_core::ObjectId;

use super::{fetch, move_object, store::S3ObjectStore, tag};

#[async_trait]
impl ObjectStore for S3ObjectStore {
    async fn fetch(&self, id: &ObjectId, max_size: u64) -> Result<FetchedObject, ObjectStoreError> {
        fetch::fetch(self.client(), id, max_size).await
    }

    async fn tag(&self, id: &ObjectId, tags: &TagSet) -> Result<(), ObjectStoreError> {
        tag::tag(self.client(), id, tags).await
    }

    async fn move_to(&self, id: &ObjectId, dest_key: &str) -> Result<(), ObjectStoreError> {
        move_object::move_to(self.client(), id, dest_key).await
    }

    async fn delete(&self, id: &ObjectId) -> Result<(), ObjectStoreError> {
        move_object::delete(self.client(), id).await
    }
}
