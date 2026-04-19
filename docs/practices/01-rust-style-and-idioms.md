# 01. Rust Style and Idioms

## Baseline expectations

Rust code MUST be idiomatic, explicit about ownership, easy to test, and hard to misuse.
Use the compiler as a design partner: encode invariants in types and make invalid states unrepresentable where practical.

## Formatting and linting

- MUST use `cargo fmt` for formatting.
- MUST use Clippy with warnings denied in CI.
- MUST not silence lints globally unless the repository documents the reason.
- MAY allow a lint locally when the code is clearer and the reason is documented near the allow.

```rust
#[allow(clippy::too_many_arguments)]
fn construct_from_legacy_wire_format(/* legacy boundary only */) {}
```

## Naming

- Types and traits: `UpperCamelCase`.
- Functions, methods, variables, modules, and files: `snake_case`.
- Constants: `SCREAMING_SNAKE_CASE`.
- Conversions should follow common Rust conventions: `as_`, `to_`, `into_`, `From`, `TryFrom`, `AsRef`, and `AsMut`.
- Iterator-producing methods should be named `iter`, `iter_mut`, or `into_iter` when they match those semantics.

## Ownership and borrowing

- Prefer borrowing over cloning when the caller can retain ownership.
- Prefer owned values at API boundaries when storing data.
- Avoid returning references tied to overly broad lifetimes when an owned result is simpler and safe.
- Use `Cow` only when it measurably improves ergonomics or performance.
- Avoid `Arc<Mutex<T>>` as a default design; first ask whether ownership can be simplified.

## Types and invariants

- Use newtypes for validated domain values.
- Keep fields private when the type has invariants.
- Prefer enums over stringly typed states.
- Prefer `Option<T>` over sentinel values.
- Prefer `Result<T, E>` over error codes.

```rust
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn parse(value: String) -> Result<Self, EmailAddressError> {
        if value.contains('@') {
            Ok(Self(value))
        } else {
            Err(EmailAddressError::MissingAtSign)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

## Error handling

- Libraries MUST return typed errors for recoverable failures.
- Applications MAY convert errors to user-facing messages at the outer boundary.
- Use `thiserror`-style domain errors in libraries when appropriate.
- Use `anyhow`-style context in binaries and operational glue when appropriate.
- Preserve source errors when they help diagnosis.
- Do not log and return the same error at multiple layers.

## Unsafe Rust

- MUST avoid `unsafe` unless there is no safe alternative that meets requirements.
- MUST isolate `unsafe` in the smallest possible module.
- MUST document safety invariants beside each `unsafe` block.
- MUST provide tests around the safe wrapper.
- MUST NOT expose unsafe internals through public safe APIs without preserving soundness.

## Async and concurrency

- Do not make a function `async` unless it awaits asynchronous work.
- Keep domain logic synchronous when it does not need I/O.
- Put async I/O in adapter or boundary modules.
- Prefer message passing or ownership transfer over shared mutable state.
- Do not hold a synchronous mutex guard across `.await`.

## Dependencies

- Add dependencies intentionally and narrowly.
- Prefer standard library features when they are clear and sufficient.
- Keep domain crates free from framework, database, HTTP, and runtime dependencies unless those are the domain.
- Do not expose third-party concrete types from public APIs unless the dependency is intentionally part of the API contract.
