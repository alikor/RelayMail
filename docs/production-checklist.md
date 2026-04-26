# Production Checklist

Use this checklist for each environment before sending real traffic.

## Domains

- [ ] Choose stream sender domains, for example `mail.example.com` for transactional mail and `news.example.com` for marketing mail.
- [ ] Verify each sender domain in every enabled provider.
- [ ] Publish DKIM records for each provider/domain pair.
- [ ] Publish SPF records that include the active providers for each stream domain.
- [ ] Publish DMARC for each stream domain, starting with monitoring mode if needed.

## Providers

- [ ] Create least-privilege Resend API key and set `RESEND_API_KEY`.
- [ ] Configure Resend webhook endpoint `/api/relaymail/webhooks/resend`.
- [ ] Configure Postmark only after account/server approval, then set `POSTMARK_SERVER_TOKEN`.
- [ ] Configure Postmark transactional message stream; use a broadcast stream only for marketing if intentionally enabled upstream.
- [ ] Configure SMTP2GO as an emergency fallback and set `SMTP2GO_API_KEY`.
- [ ] Keep AWS SES disabled with `RELAYMAIL_AWS_SES_ENABLED=false` until production sending access is approved.

## RelayMail Config

- [ ] Set `RELAYMAIL_PRIMARY_PROVIDER=resend`.
- [ ] Set `RELAYMAIL_FALLBACK_PROVIDERS=postmark,smtp2go`.
- [ ] Set `RELAYMAIL_TRANSPORT_STATE_TABLE_NAME` to a persistent DynamoDB table.
- [ ] Set per-stream allowed sender domains and default senders.
- [ ] Keep `RELAYMAIL_STREAM_MARKETING_REQUIRE_UNSUBSCRIBE=true`.
- [ ] Keep `RELAYMAIL_STREAM_MARKETING_REQUIRE_CONSENT_METADATA=true`.
- [ ] Set webhook secrets/auth variables for every enabled provider.

## Validation

- [ ] Send a transactional test message.
- [ ] Send a marketing test message with unsubscribe and consent metadata.
- [ ] Confirm provider acceptance is logged.
- [ ] Confirm webhook authentication rejects invalid requests.
- [ ] Confirm delivered/bounce/complaint events are logged.
- [ ] Confirm hard bounce and complaint events create suppressions.
- [ ] Confirm a suppressed recipient is blocked before provider send.
- [ ] Confirm fallback does not occur after provider acceptance.

## Monitoring

- [ ] Alert on sustained `relaymail_email_failures_total`.
- [ ] Alert on `relaymail_webhook_duplicate_total` spikes.
- [ ] Monitor provider rejection, hard bounce, transient bounce, complaint, and suppression rates.
- [ ] Review provider dashboards during launch and after DNS changes.
