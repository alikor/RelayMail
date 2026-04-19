# 00. Agent Contract

## Purpose

This is the mandatory behaviour contract for coding agents generating or editing Rust code.
It is written as direct instructions rather than advice.

## Non-negotiable rules

### Source file size

- MUST NOT create or leave any handwritten `.rs` source file above 100 physical lines.
- MUST split any file above 100 lines before claiming completion.
- MUST split by domain concept or responsibility, not by arbitrary line ranges.
- MUST keep `mod.rs` or facade files small; they should declare modules and re-export the public API only.
- MAY exclude generated code only when it is clearly marked as generated and is not hand edited.

### Minimal exposure

- MUST keep items private by default.
- MUST use `pub(crate)`, `pub(super)`, or `pub(in path)` instead of `pub` when external users do not need the item.
- MUST keep struct fields private unless direct field access is part of a stable public API.
- MUST expose constructors, query methods, and domain operations instead of raw internal state.
- MUST NOT re-export internal implementation details from crate roots or facade modules.

### Tests and coverage

- MUST add or update tests for changed behaviour.
- MUST preserve a minimum of 80% line coverage for the workspace.
- MUST prefer fast unit tests for domain logic.
- MUST use integration tests for external behaviour and boundary wiring.
- MUST NOT use coverage percentage as an excuse for weak assertions.

### Rust quality gates

A task is incomplete until these pass, or until the agent clearly reports what could not be run:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo llvm-cov --workspace --all-features --fail-under-lines 80
```

### Clean implementation rules

- MUST prefer clear names over comments that explain confusing code.
- MUST keep functions small and at one level of abstraction.
- MUST use `Result` for recoverable failures and `panic!` only for programmer errors or impossible states.
- MUST avoid `unwrap`, `expect`, and indexing that can panic in production paths unless the invariant is obvious and documented.
- MUST remove duplication by extracting concepts, not by hiding differences behind vague abstractions.

## Agent workflow

1. Inspect the current module boundaries before editing.
2. Identify the domain responsibility of the change.
3. Make the smallest API change that supports the behaviour.
4. Write or update tests close to the code being changed.
5. Run quality gates.
6. Split any source file above 100 lines.
7. Review public API exposure and remove unnecessary visibility.
8. Summarise what changed and what was verified.
