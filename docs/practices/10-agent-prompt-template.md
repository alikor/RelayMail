# 10. Agent Prompt Template

Use this block as a system or repository instruction for coding agents.

```text
You are editing a Rust codebase.
Follow the Rust Clean Code Agent Guide.

Hard rules:
- No handwritten Rust source file may exceed 100 physical lines.
- Split large files into domain-specific modules before finishing.
- Keep modules, types, fields, functions, and traits private unless exposure is necessary.
- Prefer `pub(crate)`, `pub(super)`, or `pub(in path)` over `pub` when possible.
- Public struct fields are forbidden unless direct construction is intentionally part of the API.
- Maintain at least 80% line coverage.
- Add or update fast unit tests for changed behaviour.
- Run or report inability to run: `cargo fmt`, `cargo clippy`, `cargo test`, and coverage.

Design rules:
- Apply SOLID in a Rust-native way using modules, traits, enums, generics, and type invariants.
- Do not add speculative traits or Java-style class hierarchies.
- Keep domain logic separate from adapters such as databases, HTTP, queues, clocks, and files.
- Prefer clear names, small functions, typed errors, and tests that specify behaviour.
- Avoid `unwrap`, `expect`, and `panic!` in production paths unless the invariant is documented.

Before finishing:
- Check all `.rs` files are at most 100 lines.
- Remove unnecessary public visibility.
- Ensure tests are meaningful and deterministic.
- Ensure the change leaves the code cleaner than it was.
```

## Suggested task response format

```text
Changed:
- ...

Design notes:
- ...

Verification:
- cargo fmt --all -- --check
- cargo clippy --workspace --all-targets --all-features -- -D warnings
- cargo test --workspace --all-features
- cargo llvm-cov --workspace --all-features --fail-under-lines 80
- checked no `.rs` file exceeds 100 lines

Caveats:
- ...
```
