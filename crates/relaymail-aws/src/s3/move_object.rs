use aws_sdk_s3::Client;
use relaymail_core::object_store::ObjectStoreError;
use relaymail_core::ObjectId;

use super::error_map::map_sdk_error;

pub(crate) async fn move_to(
    client: &Client,
    id: &ObjectId,
    dest_key: &str,
) -> Result<(), ObjectStoreError> {
    let source = format!("{}/{}", id.bucket(), id.key());
    client
        .copy_object()
        .bucket(id.bucket())
        .key(dest_key)
        .copy_source(source)
        .send()
        .await
        .map_err(|e| map_sdk_error(e, "copy_object"))?;
    client
        .delete_object()
        .bucket(id.bucket())
        .key(id.key())
        .send()
        .await
        .map_err(|e| map_sdk_error(e, "delete_object"))?;
    Ok(())
}

pub(crate) async fn delete(client: &Client, id: &ObjectId) -> Result<(), ObjectStoreError> {
    client
        .delete_object()
        .bucket(id.bucket())
        .key(id.key())
        .send()
        .await
        .map_err(|e| map_sdk_error(e, "delete_object"))?;
    Ok(())
}
