//! S3-backed `ObjectStore` implementation.

pub(crate) mod error_map;
pub(crate) mod fetch;
pub(crate) mod impl_object_store;
pub(crate) mod move_object;
pub(crate) mod store;
pub(crate) mod tag;

pub use self::store::S3ObjectStore;
