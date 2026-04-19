use metrics::{counter, histogram};

use super::descriptors::{
    EMAILS_PROCESSED_TOTAL, EMAILS_SENT_TOTAL, EMAIL_FAILURES_TOTAL, IDEMPOTENCY_SKIPS_TOTAL,
    PROCESSING_DURATION_SECONDS, SES_SEND_LATENCY_SECONDS,
};

pub fn emit_processed(service: &str, provider: &str, status: &str) {
    counter!(EMAILS_PROCESSED_TOTAL,
        "service" => service.to_string(),
        "provider" => provider.to_string(),
        "status" => status.to_string()
    )
    .increment(1);
}

pub fn emit_sent(service: &str, provider: &str) {
    counter!(EMAILS_SENT_TOTAL, "service" => service.to_string(), "provider" => provider.to_string())
        .increment(1);
}

pub fn emit_failure(service: &str, error_class: &str) {
    counter!(EMAIL_FAILURES_TOTAL,
        "service" => service.to_string(),
        "error_class" => error_class.to_string()
    )
    .increment(1);
}

pub fn emit_idempotency_skip(service: &str) {
    counter!(IDEMPOTENCY_SKIPS_TOTAL, "service" => service.to_string()).increment(1);
}

pub fn emit_send_latency(service: &str, seconds: f64) {
    histogram!(SES_SEND_LATENCY_SECONDS, "service" => service.to_string()).record(seconds);
}

pub fn emit_processing_duration(service: &str, seconds: f64) {
    histogram!(PROCESSING_DURATION_SECONDS, "service" => service.to_string()).record(seconds);
}
