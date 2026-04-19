# 06. Testing and 80% Coverage

## Minimum requirement

The workspace MUST maintain at least 80% line coverage.
New or changed domain logic MUST be covered by fast unit tests.
Integration and documentation tests are valuable, but they do not replace unit tests for core rules.

## Coverage command

Use this command unless the repository defines an equivalent stricter command:

```sh
cargo llvm-cov --workspace --all-features --fail-under-lines 80 --show-missing-lines
```

Generate a local HTML report when investigating gaps:

```sh
cargo llvm-cov --workspace --all-features --html --open
```

## Test placement

Use colocated unit tests for private domain logic:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_negative_invoice_total() {
        let result = InvoiceTotal::new(-1);
        assert!(matches!(result, Err(InvoiceError::NegativeTotal)));
    }
}
```

Use `tests/` for integration-style tests that exercise public crate behaviour.
Use documentation tests for public examples that should compile and remain accurate.

## Test quality rules

- Tests MUST assert behaviour, not implementation details.
- Tests MUST be deterministic.
- Tests MUST not require real network, real time, or external services unless marked and isolated.
- Tests SHOULD use fakes or in-memory adapters at boundaries.
- Tests SHOULD cover success, validation failure, dependency failure, and edge cases.
- Tests MUST be readable enough to act as examples.

## Coverage is not enough

80% line coverage is a floor, not a quality guarantee.
Agents MUST still test meaningful behaviours and edge cases.
A weak test that executes code without assertions does not satisfy this guide.

## What to test

Prioritize:

1. domain invariants;
2. error handling;
3. branching logic;
4. boundary mapping;
5. serialization and deserialization contracts;
6. adapter behaviour with fakes or controlled test containers when the repo supports them.

## What not to over-test

Avoid tests that only verify:

- generated code;
- trivial getters;
- implementation-private call order;
- formatting produced by third-party libraries;
- framework behaviour already covered by the framework.

## Fakes and test doubles

Prefer small fake implementations of capability traits.
The fake should live in the test module unless reused by many tests.
If a fake grows above 100 lines, split it into a test support module with a narrow purpose.

```rust
struct FixedClock(DateTimeUtc);

impl Clock for FixedClock {
    fn now(&self) -> DateTimeUtc {
        self.0
    }
}
```

## Red-green-refactor workflow

1. Write a failing test for the behaviour or bug.
2. Implement the smallest change that passes.
3. Refactor names, functions, modules, and visibility.
4. Re-run tests and coverage.
5. Split files that exceed 100 lines.
