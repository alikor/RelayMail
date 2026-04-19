# 03. File Size and Domain Module Splitting

## Hard rule

No handwritten Rust source file may exceed 100 physical lines.
This includes `src/**/*.rs`, `tests/**/*.rs`, examples, benches, and support crates.
Generated files may be excluded only when they are clearly marked and never hand edited.

## Why this rule exists

Small files force clear responsibilities.
They reduce merge conflicts, improve review quality, and make coding agents less likely to mix unrelated concerns.

## What to do when a file approaches 100 lines

At 80 lines, plan the split.
At 90 lines, split before adding more behaviour.
At 101 lines, the change is not complete.

## Split by responsibility

Good splits:

- `money.rs` for money value objects;
- `invoice.rs` for invoice entity logic;
- `invoice_lines.rs` for line-item behaviour;
- `tax_policy.rs` for tax calculation rules;
- `repository.rs` for storage capability traits;
- `postgres_repository.rs` for PostgreSQL implementation.

Poor splits:

- `part1.rs`, `part2.rs`;
- `helpers.rs` with unrelated functions;
- `misc.rs`;
- `utils.rs` that hides missing domain names;
- splitting every function into its own file without a domain concept.

## Recommended module layout

```text
src/
  lib.rs
  billing/
    mod.rs
    invoice.rs
    invoice_line.rs
    money.rs
    tax_policy.rs
    service.rs
    errors.rs
  adapters/
    mod.rs
    postgres_billing.rs
    system_clock.rs
```

`billing/mod.rs` should act as a facade and stay short.
Implementation belongs in child modules.

## Refactoring sequence for large files

1. Run tests before moving code when possible.
2. Identify cohesive groups of types and functions.
3. Create a child module for one domain concept.
4. Move private helper functions with the code that uses them.
5. Add narrow re-exports from the facade only when needed.
6. Run formatting and tests.
7. Repeat until every handwritten `.rs` file is at most 100 lines.

## Module naming rules

- Name modules after domain concepts or technical boundaries.
- Prefer singular nouns for value/entity modules: `invoice`, `customer`, `money`.
- Prefer capability names for service traits: `clock`, `id_generator`, `email_sender`.
- Prefer adapter-specific names for infrastructure: `postgres_invoice_store`, `reqwest_email_client`.
- Avoid `common`, `shared`, `helpers`, and `utils` unless the contents are truly generic and tiny.

## Line-count gate

Use a conservative physical-line check in CI:

```sh
violations="$({ find src tests examples benches -name '*.rs' -type f 2>/dev/null || true; } \
  | while read -r file; do
      lines=$(wc -l < "$file")
      if [ "$lines" -gt 100 ]; then
        echo "$file:$lines"
      fi
    done)"

if [ -n "$violations" ]; then
  echo "Rust source files over 100 lines:"
  echo "$violations"
  exit 1
fi
```

## Acceptable exceptions

Exceptions should be rare and explicit.
If a generated file, binding, or macro output cannot reasonably be split, it MUST:

- live in a generated or vendor directory;
- include a generated-file marker;
- be excluded from manual edits;
- be documented in the repository quality-gate notes.
