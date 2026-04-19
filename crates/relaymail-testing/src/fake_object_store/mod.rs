//! In-memory object store usable for tests.

pub(crate) mod store;
pub(crate) mod tag_recorder;

pub use self::store::FakeObjectStore;
pub use self::tag_recorder::TagRecord;
