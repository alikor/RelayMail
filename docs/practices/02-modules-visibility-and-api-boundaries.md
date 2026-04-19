# 02. Modules, Visibility, and API Boundaries

## Goal

Modules should make the system easier to understand, safer to change, and harder to misuse.
The public API should be small, intentional, stable, and documented.

## Visibility rules

Rust is private by default. Agents MUST preserve that advantage.

Use the narrowest visibility that works:

1. Private item: default choice.
2. `pub(super)`: visible only to the parent module.
3. `pub(in crate::some_path)`: visible only inside a specific module tree.
4. `pub(crate)`: visible anywhere in the crate but not outside it.
5. `pub`: visible to downstream crates and therefore part of the API contract.

## Facade modules

A facade module should be small and declarative.
It should declare child modules and re-export only the types or functions users need.

```rust
mod invoice;
mod money;
mod service;

pub use invoice::Invoice;
pub use money::Money;
pub use service::{BillingError, BillingService};
```

Do not place business logic in a facade when it would push the file over the 100-line rule.
Move logic into focused child modules.

## Struct and enum exposure

- Public structs SHOULD have private fields unless direct construction is intended.
- Public enums expose all variants; do not make an enum public if variants are internal implementation details.
- Prefer a public opaque struct with query methods when the representation may change.
- Use `#[non_exhaustive]` for public enums or structs when downstream exhaustive construction or matching should remain flexible.

## Trait exposure

Expose a trait only when callers need to provide or choose implementations.
Do not create traits solely for every struct.

Good reasons to expose a trait:

- the domain depends on a capability implemented by adapters;
- tests need a simple fake at a boundary;
- multiple implementations are expected;
- users need extension points.

Bad reasons:

- every type must have an interface by habit;
- mocking private details;
- hiding a poorly named concrete type;
- making speculative future flexibility.

## Crate roots

`lib.rs` and `main.rs` MUST stay thin.

`lib.rs` should:

- declare top-level modules;
- re-export stable public API;
- contain crate-level documentation;
- avoid business logic.

`main.rs` should:

- parse configuration and CLI input;
- construct dependencies;
- call application entry points;
- avoid domain logic.

## Boundary placement

Keep direction of dependency clear:

- Domain modules know domain concepts.
- Application modules orchestrate use cases.
- Adapter modules know databases, queues, HTTP clients, file systems, and external services.
- Presentation modules know CLI, HTTP routes, UI, or serialization formats.

Domain code MUST NOT depend on adapter implementation details.
If domain logic needs time, randomness, storage, or external I/O, depend on a narrow trait owned by the domain or application layer.

## API review checklist

Before finishing a change, ask:

- Can this item be private?
- Can this `pub` be `pub(crate)` or `pub(super)`?
- Are all public fields necessary?
- Does this public trait have a real consumer?
- Are third-party concrete types leaking through the domain API?
- Is the public name domain-specific and stable?
- Would a downstream user be forced to change if this internal detail changed?
