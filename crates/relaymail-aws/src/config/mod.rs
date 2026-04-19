//! AWS SDK config loading.

pub(crate) mod loader;

pub use self::loader::{load_shared_aws_config, AwsConnectOptions};
