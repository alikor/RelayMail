use relaymail_core::object_store::{FetchedObject, ObjectStore};

use super::error::StageError;
use super::event_parser::ObjectRef;

pub(crate) async fn fetch(
    store: &dyn ObjectStore,
    object: &ObjectRef,
    max_size: u64,
) -> Result<FetchedObject, StageError> {
    Ok(store.fetch(&object.object, max_size).await?)
}
