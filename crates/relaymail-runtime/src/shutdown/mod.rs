//! Shutdown signal handling.

pub(crate) mod signal;
pub(crate) mod token;

pub use self::signal::install_shutdown_handler;
pub use self::token::ShutdownToken;
