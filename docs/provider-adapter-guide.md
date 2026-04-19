# Provider adapter guide

This is the checklist for adding a new delivery provider (Mailgun,
Postmark, SparkPost, SendGrid, custom SMTP) to RelayMail without
touching the domain core.

## Rules

- Implement `relaymail_delivery::EmailSender`.
- Put provider-specific SDK/HTTP/SMTP code in **one crate** named
  `relaymail-<provider>` (e.g. `relaymail-mailgun`).
- Do **not** introduce a dependency from `relaymail-core`,
  `relaymail-email`, or `relaymail-delivery` to that crate.
- Add a `relaymail-<provider>` crate entry to the root
  `[workspace.dependencies]` with a local `path =` entry.
- The new crate's public API must be its sender struct plus a small
  `RuntimeConfig` that the binary's `wire` module can populate from
  env vars.

## Minimum files per new provider

```
crates/relaymail-mailgun/
  Cargo.toml
  src/
    lib.rs              # module decls + public re-exports (thin)
    sender.rs           # MailgunSender struct + ctor + runtime config
    request.rs          # pure fn that builds the HTTP request
    response.rs         # pure fn that maps the response -> SendResult
    error.rs            # pure fn that maps provider errors -> SendError
    impl_email_sender.rs # #[async_trait] impl EmailSender for MailgunSender
```

Each file stays under 100 physical lines (see
`docs/practices/03-file-size-and-domain-module-splitting.md`).

## Testing

- Unit-test `request.rs`, `response.rs`, and `error.rs` with crafted
  inputs. These are pure — no network.
- Keep SDK call sites thin enough that low coverage there doesn't
  push the workspace under the 80% floor.
- If the provider ships an official SDK, test mapping of a sample
  error into each `SendError` variant; review
  `relaymail_delivery::error` for the target classifications.
- Avoid provider integration tests in CI. Behind a feature flag with
  `#[ignore]` is fine.

## Wiring into a binary

- The binary (today, `relaymail-email-ses`; future, an API gateway)
  decides which provider to use based on config.
- The binary constructs the provider sender inside `wire/`:

```rust
let sender: Arc<dyn EmailSender> = if cfg.provider == "mailgun" {
    Arc::new(MailgunSender::new(...))
} else {
    Arc::new(SesSender::new(...))
};
```

## Observability

- Add a label value `provider=<name>` wherever `provider_label` is
  emitted; no new metric names, just a new label value.
- Log fields stay the same; swap `ses_message_id` for a generic
  `provider_message_id` if the provider id is not SES-specific.

## Forbidden

- New enum variants in `relaymail-core` or `relaymail-delivery` to
  "represent" the provider — the abstraction works because the domain
  doesn't know which provider is in play.
- Leaking HTTP / SDK types through the public API of the new crate.
