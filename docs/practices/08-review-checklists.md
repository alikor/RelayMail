# 08. Review Checklists

## Pre-change checklist

Before editing code, answer:

- What domain responsibility is being changed?
- Which module owns that responsibility?
- Which public APIs, if any, must change?
- Which tests describe the current expected behaviour?
- Which files are already near 100 lines?

## Implementation checklist

During implementation:

- Keep functions small and named by intent.
- Keep types responsible for their invariants.
- Keep I/O and framework details out of domain modules.
- Use the narrowest visibility possible.
- Avoid speculative traits and abstractions.
- Add tests beside the logic they specify.
- Split files before they exceed 100 lines.

## Public API checklist

For every `pub` item:

- Is it needed by downstream crates?
- Is its name stable and domain-specific?
- Are its fields private unless intentionally exposed?
- Does it leak implementation details?
- Does it require documentation or examples?
- Could visibility be reduced to `pub(crate)` or `pub(super)`?

## SOLID checklist

- SRP: Does each module have one reason to change?
- OCP: Are expected variations isolated behind the right enum or trait?
- LSP: Can every trait implementation satisfy the same behavioural contract?
- ISP: Are traits small enough that consumers do not depend on unused methods?
- DIP: Does domain/application policy avoid depending on adapter details?

## Clean Code checklist

- Are names clear enough to reduce comments?
- Does each function do one thing at one level of abstraction?
- Is duplication removed only when it is the same concept?
- Are errors handled completely without hiding the main path?
- Are comments limited to why, constraints, safety, or trade-offs?
- Did the change leave the code easier to read?

## Test checklist

- Are changed behaviours covered by unit tests?
- Do tests assert meaningful outcomes?
- Are edge cases and failure paths covered?
- Are tests deterministic and fast?
- Is coverage still at least 80%?
- Are integration tests used for boundaries rather than private details?

## Final gate checklist

Before reporting completion:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo llvm-cov --workspace --all-features --fail-under-lines 80
```

Also confirm:

- no handwritten `.rs` file is above 100 lines;
- no unnecessary `pub` remains;
- no new `unwrap` or `expect` appears in production paths without a documented invariant;
- no generated or vendored code was hand edited;
- no domain module depends on concrete infrastructure.
