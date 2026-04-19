//! Clock abstraction.
//!
//! Consumers of time must accept `Arc<dyn Clock>` so tests can inject a fake
//! clock and production uses [`SystemClock`].

pub(crate) mod contract;
pub(crate) mod system_clock;

pub use self::contract::Clock;
pub use self::system_clock::SystemClock;
