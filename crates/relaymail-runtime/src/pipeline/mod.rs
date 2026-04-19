//! Provider-agnostic processing pipeline.

pub(crate) mod config;
pub(crate) mod ctx;
pub(crate) mod error;
pub(crate) mod event_parser;
pub(crate) mod failed;
pub(crate) mod outcome;
pub(crate) mod process;
pub(crate) mod stage_claim;
pub(crate) mod stage_dispose;
pub(crate) mod stage_fetch;
pub(crate) mod stage_filter;
pub(crate) mod stage_send;
pub(crate) mod stage_validate;
pub(crate) mod success;

pub use self::config::{FailureDispositionMode, ProcessingConfig, SuccessDispositionMode};
pub use self::ctx::PipelineCtx;
pub use self::error::StageError;
pub use self::event_parser::{EventParseError, EventParser, ObjectRef};
pub use self::outcome::PipelineOutcome;
pub use self::process::process_envelope;
