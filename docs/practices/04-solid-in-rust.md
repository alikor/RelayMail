# 04. SOLID in Rust

## Rust-native interpretation

SOLID is useful in Rust when treated as dependency and responsibility guidance.
Do not translate it into unnecessary class hierarchies.
Prefer modules, traits, enums, generics, ownership, and type invariants.

## S — Single Responsibility Principle

A module, type, or function should have one reason to change.
Group things that change for the same reason; separate things that change for different reasons.

Rust application:

- Put domain rules in domain modules.
- Put I/O and framework details in adapters.
- Keep validation near the value object it protects.
- Keep orchestration in use-case services.
- Split any file that mixes persistence, presentation, and business rules.

Agent check:

- Can I describe this module in one sentence without using “and then”?
- Would a database change require editing domain rules?
- Would a tax-rule change require editing HTTP routing?

## O — Open/Closed Principle

Code should be easy to extend for expected variation without modifying stable, tested policy code.

Rust application:

- Use traits for capabilities with multiple implementations.
- Use enums for closed sets of domain variants.
- Use pattern matching when the variant set is intentionally closed.
- Use generics for static dispatch and `dyn Trait` for runtime selection.
- Avoid speculative extension points before real variation exists.

```rust
pub trait Clock {
    fn now(&self) -> DateTimeUtc;
}

pub struct BillingService<C> {
    clock: C,
}

impl<C: Clock> BillingService<C> {
    pub fn new(clock: C) -> Self {
        Self { clock }
    }
}
```

## L — Liskov Substitution Principle

Every implementation of a trait must honour the trait contract.
Callers should not need special cases for surprising implementers.

Rust application:

- Document trait expectations.
- Do not implement a trait with hidden stronger preconditions.
- Do not return bogus values to satisfy a trait shape.
- Do not panic from trait methods unless the trait contract permits it.
- Prefer separate traits when one implementation cannot honestly support every method.

Agent check:

- Would all implementations satisfy the same tests?
- Does any implementation require caller-specific branching?
- Is a no-op implementation hiding a missing concept?

## I — Interface Segregation Principle

Clients should not depend on methods they do not use.
In Rust, prefer small role-focused traits.

Poor trait:

```rust
pub trait UserRepository {
    fn save(&self, user: &User) -> Result<(), Error>;
    fn find(&self, id: UserId) -> Result<User, Error>;
    fn delete(&self, id: UserId) -> Result<(), Error>;
    fn export_csv(&self) -> Result<String, Error>;
}
```

Better split:

```rust
pub trait FindUser {
    fn find(&self, id: UserId) -> Result<User, Error>;
}

pub trait SaveUser {
    fn save(&self, user: &User) -> Result<(), Error>;
}
```

## D — Dependency Inversion Principle

High-level policy should not depend on low-level implementation details.
Both should depend on abstractions owned near the policy that needs them.

Rust application:

- Define capability traits in the domain or application module that consumes them.
- Implement those traits in adapter modules.
- Keep database clients, HTTP clients, runtime handles, and framework request types outside the domain.
- Use dependency injection through constructors.
- Avoid global mutable singletons.

## SOLID caveat

Abstractions add cost.
Create a trait when it protects a boundary, improves testability, or models real variation.
Do not create a trait simply because a single concrete type exists.
