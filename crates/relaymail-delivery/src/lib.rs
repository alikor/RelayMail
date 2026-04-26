//! Provider-agnostic delivery abstraction for RelayMail.
//!
//! Add a new provider by implementing [`EmailSender`] in an adapter crate.

pub mod capabilities;
pub mod error;
pub mod normalized;
pub mod request;
pub mod result;
pub mod sender;

pub use self::capabilities::ProviderCapabilities;
pub use self::error::SendError;
pub use self::normalized::{
    EmailAddress, EmailAttachment, EmailEventType, EmailSendRequest, EmailSendResult,
    EmailSendStatus,
};
pub use self::request::SendRequest;
pub use self::result::SendResult;
pub use self::sender::EmailSender;
