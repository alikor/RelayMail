//! Application entry point: loads config, wires dependencies, runs the
//! processing loop until shutdown.

pub(crate) mod consumer_loop;
pub(crate) mod run;

pub use self::run::run;
