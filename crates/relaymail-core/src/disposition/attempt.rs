/// Count of processing attempts for a given message.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct AttemptCount(u32);

impl AttemptCount {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn first() -> Self {
        Self(1)
    }

    pub fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }

    pub fn value(self) -> u32 {
        self.0
    }

    pub fn is_exhausted(self, max: u32) -> bool {
        self.0 >= max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_and_checks_exhaustion() {
        let a = AttemptCount::first();
        assert_eq!(a.value(), 1);
        let b = a.next().next();
        assert_eq!(b.value(), 3);
        assert!(b.is_exhausted(3));
        assert!(!b.is_exhausted(4));
    }
}
