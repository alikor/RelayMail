use tracing::info;

/// Emit a startup notice that the worker is running without sending real mail.
pub(crate) fn log_dry_run_notice(service_name: &str) {
    info!(
        target: "relaymail_email_ses::dry_run",
        service = %service_name,
        "RELAYMAIL_DRY_RUN=true: pipeline will fetch + validate but not call SES."
    );
}
