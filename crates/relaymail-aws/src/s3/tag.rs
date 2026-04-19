use aws_sdk_s3::types::{Tag, Tagging};
use aws_sdk_s3::Client;
use relaymail_core::object_store::{ObjectStoreError, TagSet};
use relaymail_core::ObjectId;

use super::error_map::map_sdk_error;

pub(crate) async fn tag(
    client: &Client,
    id: &ObjectId,
    tags: &TagSet,
) -> Result<(), ObjectStoreError> {
    let payload = build_tagging(tags)?;
    client
        .put_object_tagging()
        .bucket(id.bucket())
        .key(id.key())
        .tagging(payload)
        .send()
        .await
        .map_err(|e| map_sdk_error(e, "put_object_tagging"))
        .map(|_| ())
}

fn build_tagging(tags: &TagSet) -> Result<Tagging, ObjectStoreError> {
    let mut builder = Tagging::builder();
    for (k, v) in tags.entries() {
        let tag = Tag::builder()
            .key(k)
            .value(v)
            .build()
            .map_err(|_| ObjectStoreError::Permanent("invalid tag entry".to_string()))?;
        builder = builder.tag_set(tag);
    }
    builder
        .build()
        .map_err(|_| ObjectStoreError::Permanent("invalid tag set".to_string()))
}
