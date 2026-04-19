# 09. Rust Snippets and Patterns

These snippets are intentionally small.
Do not copy them blindly; adapt names to the domain.

## Thin `lib.rs`

```rust
//! Billing domain library.

pub mod billing;

pub use billing::{BillingError, BillingService, Invoice, Money};
```

## Facade module

```rust
mod invoice;
mod money;
mod service;

pub use invoice::Invoice;
pub use money::Money;
pub use service::{BillingError, BillingService};
```

## Private fields with constructor

```rust
pub struct Money {
    amount_minor: i64,
    currency: Currency,
}

impl Money {
    pub fn new(amount_minor: i64, currency: Currency) -> Self {
        Self {
            amount_minor,
            currency,
        }
    }

    pub fn amount_minor(&self) -> i64 {
        self.amount_minor
    }
}
```

## Narrow capability trait

```rust
pub trait InvoiceStore {
    fn save(&self, invoice: &Invoice) -> Result<(), StoreError>;
}
```

## Application service with injected dependency

```rust
pub struct BillingService<S> {
    store: S,
}

impl<S: InvoiceStore> BillingService<S> {
    pub fn new(store: S) -> Self {
        Self { store }
    }

    pub fn issue(&self, invoice: Invoice) -> Result<(), BillingError> {
        invoice.validate()?;
        self.store.save(&invoice)?;
        Ok(())
    }
}
```

## Error conversion at a boundary

```rust
impl From<StoreError> for BillingError {
    fn from(source: StoreError) -> Self {
        Self::Storage { source }
    }
}
```

## Unit test pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_total_invoice_is_invalid() {
        let invoice = Invoice::new(Money::zero(Currency::Gbp));
        assert!(matches!(invoice.validate(), Err(BillingError::ZeroTotal)));
    }
}
```

## Local lint exception with reason

```rust
#[allow(clippy::module_name_repetitions)]
pub struct BillingService {
    // Name is intentionally explicit in public API docs.
}
```

## Do not expose adapter details

Poor domain API:

```rust
pub fn load_invoice(pool: PgPool, id: Uuid) -> Result<Invoice, sqlx::Error> {
    todo!()
}
```

Better shape:

```rust
pub trait LoadInvoice {
    fn load(&self, id: InvoiceId) -> Result<Invoice, LoadInvoiceError>;
}
```
