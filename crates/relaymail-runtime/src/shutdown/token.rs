use tokio_util::sync::CancellationToken;

/// Thin wrapper around [`CancellationToken`] so downstream crates don't need
/// to depend on `tokio_util` directly.
#[derive(Clone, Debug, Default)]
pub struct ShutdownToken {
    inner: CancellationToken,
}

impl ShutdownToken {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&self) {
        self.inner.cancel();
    }

    pub fn is_cancelled(&self) -> bool {
        self.inner.is_cancelled()
    }

    pub async fn cancelled(&self) {
        self.inner.cancelled().await
    }

    pub fn raw(&self) -> &CancellationToken {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn cancels_when_asked() {
        let t = ShutdownToken::new();
        assert!(!t.is_cancelled());
        t.cancel();
        assert!(t.is_cancelled());
        t.cancelled().await;
    }
}
