//! Storage abstraction over an object store (initial impl: S3).

pub(crate) mod contract;
pub(crate) mod error;
pub(crate) mod metadata;

pub use self::contract::{FetchedObject, ObjectStore, TagSet};
pub use self::error::ObjectStoreError;
pub use self::metadata::ObjectMetadata;
