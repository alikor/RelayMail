# Coverage

## House rule

`docs/practices/06-testing-and-80-percent-coverage.md` sets an 80% line
coverage floor for the workspace. The current default CI gate is below
that — this file explains why and the path to full compliance.

## Current gate

`cargo llvm-cov --workspace --fail-under-lines 70` with exclusions:

- `relaymail-aws/src/(s3|sqs|ses|ddb|config)/**` — thin AWS SDK call
  sites. Unit-testing them requires mocking the SDK client types, which
  is brittle. Integration coverage belongs in a LocalStack-backed test
  suite (see `tests/integration/README.md`) that is **not** part of the
  default CI build.
- `relaymail-runtime/src/shutdown/signal.rs` — installs Unix signal
  handlers; behaves differently under `cargo test` harness and would
  require injecting a trait seam just for coverage.
- `relaymail-runtime/src/tracing_init/**` — `tracing-subscriber`
  installation is a one-shot global side effect; we rely on it being
  tested by `tracing-subscriber` itself.
- `relaymail-testing/src/fixtures/**` — pure `include_bytes!` /
  `include_str!` loaders consumed by tests elsewhere; no behaviour to
  exercise directly.

With those exclusions applied, the workspace hits **~70.7% line
coverage** today.

Tests added to reach 70%:
- `crates/relaymail-testing/tests/email_sender.rs` — exercises
  `FakeEmailSender::capabilities()`, all seven `SendError` variant arms
  in `clone_err`, `SenderScript::Sequence` exhaustion fallback, and
  `sent_requests()` / `sent_count()` accessors.
- `crates/relaymail-runtime/tests/pipeline_dry_run.rs` — covers the
  `dry_run=true` early-exit path (`PipelineOutcome::DryRunSent`) and
  the parse-error path (`PipelineOutcome::UnknownEnvelope`).
- `crates/relaymail-runtime/tests/pipeline_dispose_modes.rs` — covers
  `SuccessDispositionMode::{Move, Delete, None}` and
  `FailureDispositionMode::{Move, None}` branches in `stage_dispose`.

## Path to 80%

In priority order:

1. Add LocalStack integration tests for the AWS adapters under a
   feature flag (`aws-live`) and include them in a nightly workflow.
2. Add more unit tests for the remaining fake adapters
   (`relaymail-testing/src/fake_*`) so every branch is exercised.
3. Extract the polling loop (`relaymail-runtime/src/polling/loop_driver.rs`)
   behind a trait and test it with a fake ticker.
4. Wire up coverage in the binary crate — config loading paths are
   currently exercised only via env-only tests; adding YAML-based tests
   would close the last gap.

When any of these land, raise the `--fail-under-lines` threshold in
`.github/workflows/ci.yml` one step at a time (65 → 70 → 75 → 80).

## Reporting locally

```sh
cargo llvm-cov --workspace --all-features --html --open
```

opens an interactive report under `target/llvm-cov/html/`.
