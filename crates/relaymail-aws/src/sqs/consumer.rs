use aws_sdk_sqs::Client;

/// Long-poll tuning for the SQS consumer.
#[derive(Clone, Debug)]
pub struct SqsConsumerConfig {
    pub queue_url: String,
    pub max_messages: i32,
    pub wait_time_seconds: i32,
    pub visibility_timeout_seconds: i32,
}

/// SQS-backed message source. Use [`MessageSource::receive`] to long-poll.
#[derive(Clone, Debug)]
pub struct SqsConsumer {
    client: Client,
    config: SqsConsumerConfig,
}

impl SqsConsumer {
    pub fn new(client: Client, config: SqsConsumerConfig) -> Self {
        Self { client, config }
    }

    pub(crate) fn client(&self) -> &Client {
        &self.client
    }

    pub(crate) fn config(&self) -> &SqsConsumerConfig {
        &self.config
    }
}
