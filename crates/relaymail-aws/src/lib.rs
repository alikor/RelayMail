//! AWS adapter implementations for RelayMail capability traits.
//!
//! Each sub-module provides one adapter. Domain types and traits live in
//! `relaymail-core`, `relaymail-email`, and `relaymail-delivery`; this crate
//! only glues them to the AWS SDK.

pub mod config;
pub mod ddb;
pub mod events;
pub mod s3;
pub mod ses;
pub mod sqs;
pub mod tagging;
pub mod transport;

pub use self::config::load_shared_aws_config;
pub use self::ddb::DynamoIdempotencyStore;
pub use self::events::{S3EventParser, S3ObjectEvent};
pub use self::s3::S3ObjectStore;
pub use self::ses::SesSender;
pub use self::sqs::SqsConsumer;
pub use self::transport::DynamoTransportStore;
