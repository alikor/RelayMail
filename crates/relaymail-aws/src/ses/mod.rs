//! SES v2 `EmailSender` implementation.

pub(crate) mod error_map;
pub(crate) mod impl_email_sender;
pub(crate) mod sender;

pub use self::sender::{SesRuntimeConfig, SesSender};
