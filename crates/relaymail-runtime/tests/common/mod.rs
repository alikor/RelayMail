#![allow(unused_imports, dead_code)]

pub mod fixture;
pub mod parser;

pub use self::fixture::{config, fixture, Fixture};
pub use self::parser::{object_ref, StaticEventParser};
