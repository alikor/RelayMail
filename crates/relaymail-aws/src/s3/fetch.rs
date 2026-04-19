use aws_sdk_s3::Client;
use relaymail_core::object_store::{FetchedObject, ObjectStoreError};
use relaymail_core::{ObjectId, ObjectMetadata};

use super::error_map::map_sdk_error;

pub(crate) async fn fetch(
    client: &Client,
    id: &ObjectId,
    max_size: u64,
) -> Result<FetchedObject, ObjectStoreError> {
    let head = client
        .head_object()
        .bucket(id.bucket())
        .key(id.key())
        .send()
        .await
        .map_err(|e| map_sdk_error(e, "head_object"))?;
    let size = head.content_length().unwrap_or_default() as u64;
    if size > max_size {
        return Err(ObjectStoreError::TooLarge {
            actual: size,
            limit: max_size,
        });
    }
    let get = client
        .get_object()
        .bucket(id.bucket())
        .key(id.key())
        .send()
        .await
        .map_err(|e| map_sdk_error(e, "get_object"))?;
    let data = get
        .body
        .collect()
        .await
        .map_err(|e| ObjectStoreError::Transient(format!("stream: {e}")))?
        .into_bytes();
    let metadata = ObjectMetadata::new(
        head.e_tag().unwrap_or("").trim_matches('"').to_string(),
        size,
    );
    Ok(FetchedObject {
        bytes: data,
        metadata,
    })
}
