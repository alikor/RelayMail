//! In-memory `MessageSource` usable for tests.

pub(crate) mod builder;
pub(crate) mod source;

pub use self::builder::FakeEnvelopeBuilder;
pub use self::source::FakeMessageSource;
