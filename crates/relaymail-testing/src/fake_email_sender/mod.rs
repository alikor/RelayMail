//! Configurable fake `EmailSender` for pipeline tests.

pub(crate) mod inspect;
pub mod script;
pub(crate) mod sender;

pub use self::script::{SenderScript, Step};
pub use self::sender::FakeEmailSender;
