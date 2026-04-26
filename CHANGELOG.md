# Changelog

## 1.0.0

Major release: RelayMail is now a generic, provider-agnostic email
transport worker.

### Added

- Resend, Postmark, and SMTP2GO REST provider adapters.
- Configurable provider chain with pre-acceptance fallback.
- Configurable `transactional` and `marketing` streams with sender-domain
  allowlists.
- Marketing transport compliance checks for unsubscribe and consent
  metadata.
- Provider webhook endpoints for event normalization, deduplication, and
  suppression creation.
- DynamoDB transport-state table for send attempts, message logs, events,
  and suppressions.
- Production checklist covering DNS, provider setup, webhook auth, and
  launch validation.

### Changed

- SES remains available but is disabled unless `RELAYMAIL_AWS_SES_ENABLED=true`.
- S3 success tags now store generic `relaymail-provider-message-id`.
- Documentation and examples use generic placeholder domains and no
  customer-specific references.

### Notes

- Upstream systems remain responsible for campaign/list/contact management
  and consent collection. RelayMail enforces transport-level compliance and
  suppression behavior only.
- Production is not ready until sender domains, SPF/DKIM/DMARC, provider
  accounts, API keys, webhooks, and DynamoDB tables are configured.
