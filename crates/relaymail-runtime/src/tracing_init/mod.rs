//! Tracing/logging initialization (JSON in prod, pretty in dev).

pub(crate) mod init;
pub(crate) mod layers;

pub use self::init::install_tracing;
