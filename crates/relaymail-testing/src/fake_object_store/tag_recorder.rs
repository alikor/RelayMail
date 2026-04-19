use relaymail_core::object_store::TagSet;
use relaymail_core::ObjectId;

/// Single recorded tag/move/delete operation.
#[derive(Clone, Debug)]
pub struct TagRecord {
    pub object: ObjectId,
    pub tags: TagSet,
}

impl TagRecord {
    pub fn new(object: ObjectId, tags: TagSet) -> Self {
        Self { object, tags }
    }
}
