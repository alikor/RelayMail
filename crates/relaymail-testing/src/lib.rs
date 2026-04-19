//! Fakes and fixtures for RelayMail tests.
//!
//! Not published. Consumed only by `dev-dependencies` of the workspace crates.

pub mod fake_clock;
pub mod fake_email_sender;
pub mod fake_idempotency_store;
pub mod fake_message_source;
pub mod fake_object_store;
pub mod fixtures;

pub use self::fake_clock::FakeClock;
pub use self::fake_email_sender::{FakeEmailSender, SenderScript, Step};
pub use self::fake_idempotency_store::FakeIdempotencyStore;
pub use self::fake_message_source::{FakeEnvelopeBuilder, FakeMessageSource};
pub use self::fake_object_store::{FakeObjectStore, TagRecord};
