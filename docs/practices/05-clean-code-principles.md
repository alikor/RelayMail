# 05. Clean Code Principles for Rust

## Intent

These rules adapt widely used Clean Code principles to Rust.
They emphasize readability, small functions, meaningful names, complete error handling, tests, and continuous refactoring.

## Meaningful names

- Choose names that reveal intent.
- Use domain language instead of generic technical terms.
- Avoid abbreviations unless they are standard in the domain.
- Avoid vague names such as `data`, `thing`, `manager`, `processor`, and `helper`.
- Rename before adding comments that explain what a name should have said.

Poor:

```rust
fn calc(x: i64, y: i64) -> i64 {
    x - y
}
```

Better:

```rust
fn outstanding_balance(invoice_total: Money, paid: Money) -> Money {
    invoice_total - paid
}
```

## Small functions

Functions SHOULD be small enough to understand at a glance.
A function should usually do one thing at one level of abstraction.

Split when a function:

- mixes validation, calculation, persistence, and presentation;
- has deeply nested branching;
- needs comments to explain sections;
- is difficult to name precisely;
- makes a file exceed 100 lines.

## One level of abstraction

Do not mix high-level policy and low-level details in the same function.

Poor:

```rust
fn register_user(input: Input, pool: PgPool) -> Result<Response, Error> {
    // validates email, hashes password, builds SQL, maps HTTP response
    todo!()
}
```

Better structure:

- parse request at the boundary;
- validate domain values in domain types;
- run use case in an application service;
- persist through a narrow repository trait;
- map result to HTTP response at the boundary.

## Comments

Prefer self-explaining code.
Use comments for:

- why a non-obvious decision exists;
- external constraints;
- safety invariants;
- performance trade-offs;
- protocol quirks.

Do not use comments to repeat what code says.
Do not leave commented-out code.

## Error handling

- Handle errors at the layer with enough context to decide.
- Add context when crossing boundaries.
- Keep domain errors precise.
- Do not swallow errors.
- Do not convert all failures to strings too early.
- Do not let error handling obscure the main path.

## Duplication

Remove duplication when it represents the same concept.
Do not over-abstract code that only looks similar but changes for different reasons.
Prefer a well-named function, type, or module over a generic helper.

## Boundaries

External systems are boundaries.
Treat databases, HTTP APIs, queues, clocks, random number generators, file systems, and environment variables as adapters.
Keep their details out of core domain logic.

## Tests as clean code

Tests should be readable specifications.

- Test names should describe behaviour.
- Arrange, act, and assert should be visually clear.
- Avoid brittle tests that mirror implementation details.
- Use test builders when setup becomes noisy.
- Keep test helpers small and domain-specific.

## Refactoring rule

Leave the code cleaner than you found it.
For every change, remove at least one avoidable ambiguity, duplication, or unnecessary exposure when it is safe to do so.
