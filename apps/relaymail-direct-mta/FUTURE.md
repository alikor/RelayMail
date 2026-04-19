# Future scope — relaymail-direct-mta

Responsibilities (none implemented yet):

- SMTP connection management and outbound TLS
- DNS/MX resolution with caching and fallback
- DKIM signing (workspace-wide policy)
- Bounce processing and hard/soft classification
- Reputation management per IP pool
- Suppression lists (bounces + complaints + manual)
- Rate limiting per destination domain
- Retry scheduling with domain-specific backoff
- Delivery analytics emitted to `relaymail-events`

Integration plan:

- Implement `relaymail_delivery::EmailSender` so `relaymail-runtime`'s
  pipeline can switch provider with config only.
- Keep SMTP/DNS/TLS code inside this crate. Do NOT leak to
  `relaymail-core`, `relaymail-email`, or `relaymail-delivery`.
- Reuse `relaymail-runtime` worker pool, shutdown, HTTP endpoints, and
  metrics.

This crate has no `Cargo.toml` on purpose: it is NOT yet a workspace
member. When it gains real code, add it to the root `Cargo.toml`'s
`[workspace] members` array.
