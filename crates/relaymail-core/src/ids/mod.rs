//! Domain identifiers.

pub(crate) mod message;
pub(crate) mod object;
pub(crate) mod tenant;

pub use self::message::MessageId;
pub use self::object::ObjectId;
pub use self::tenant::TenantId;
