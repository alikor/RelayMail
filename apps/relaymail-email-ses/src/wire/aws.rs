use aws_sdk_dynamodb::Client as DynamoClient;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_sesv2::Client as SesClient;
use aws_sdk_sqs::Client as SqsClient;
use relaymail_aws::config::{load_shared_aws_config, AwsConnectOptions};

use crate::config::AppConfig;

pub(crate) struct AwsClients {
    pub s3: S3Client,
    pub sqs: SqsClient,
    pub ses: SesClient,
    pub dynamo: DynamoClient,
}

pub(crate) async fn build_aws_clients(cfg: &AppConfig) -> AwsClients {
    let options = AwsConnectOptions {
        region: cfg.aws.region.clone(),
        endpoint_url: cfg.aws.endpoint_url.clone(),
    };
    let shared = load_shared_aws_config(options).await;
    AwsClients {
        s3: S3Client::new(&shared),
        sqs: SqsClient::new(&shared),
        ses: SesClient::new(&shared),
        dynamo: DynamoClient::new(&shared),
    }
}
