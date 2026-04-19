use metrics::{describe_counter, describe_histogram};

pub(crate) const EMAILS_PROCESSED_TOTAL: &str = "relaymail_emails_processed_total";
pub(crate) const EMAILS_SENT_TOTAL: &str = "relaymail_emails_sent_total";
pub(crate) const EMAIL_FAILURES_TOTAL: &str = "relaymail_email_failures_total";
pub(crate) const S3_DOWNLOAD_BYTES_TOTAL: &str = "relaymail_s3_download_bytes_total";
pub(crate) const SES_SEND_LATENCY_SECONDS: &str = "relaymail_ses_send_latency_seconds";
pub(crate) const SQS_MESSAGES_RECEIVED_TOTAL: &str = "relaymail_sqs_messages_received_total";
pub(crate) const SQS_MESSAGES_DELETED_TOTAL: &str = "relaymail_sqs_messages_deleted_total";
pub(crate) const PROCESSING_DURATION_SECONDS: &str = "relaymail_processing_duration_seconds";
pub(crate) const IDEMPOTENCY_SKIPS_TOTAL: &str = "relaymail_idempotency_skips_total";
pub(crate) const DRY_RUN_TOTAL: &str = "relaymail_dry_run_total";

pub(crate) fn describe_all() {
    describe_counter!(EMAILS_PROCESSED_TOTAL, "Emails processed by outcome.");
    describe_counter!(EMAILS_SENT_TOTAL, "Emails accepted by the provider.");
    describe_counter!(EMAIL_FAILURES_TOTAL, "Email failures by error class.");
    describe_counter!(
        S3_DOWNLOAD_BYTES_TOTAL,
        "Bytes downloaded from the object store."
    );
    describe_histogram!(
        SES_SEND_LATENCY_SECONDS,
        "SES send-call latency in seconds."
    );
    describe_counter!(
        SQS_MESSAGES_RECEIVED_TOTAL,
        "Envelopes received from the message source."
    );
    describe_counter!(
        SQS_MESSAGES_DELETED_TOTAL,
        "Envelopes acked (deleted) from the source."
    );
    describe_histogram!(
        PROCESSING_DURATION_SECONDS,
        "End-to-end processing duration in seconds."
    );
    describe_counter!(
        IDEMPOTENCY_SKIPS_TOTAL,
        "Already-claimed idempotency hits skipped."
    );
    describe_counter!(DRY_RUN_TOTAL, "Messages processed in dry-run mode.");
}
