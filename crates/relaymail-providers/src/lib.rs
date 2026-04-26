//! REST provider adapters for RelayMail.

mod common;
mod postmark;
mod resend;
mod smtp2go;

pub use self::postmark::{PostmarkConfig, PostmarkSender};
pub use self::resend::{ResendConfig, ResendSender};
pub use self::smtp2go::{Smtp2GoConfig, Smtp2GoSender};
