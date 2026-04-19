//! Raw (owned) email bytes.

pub(crate) mod message;
pub(crate) mod size_limits;

pub use self::message::RawEmail;
pub use self::size_limits::MaxSize;
