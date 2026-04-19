use std::future::Future;
use std::sync::Arc;

use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::shutdown::ShutdownToken;

/// Bounded pool of async jobs backed by a tokio semaphore.
#[derive(Debug)]
pub struct WorkerPool {
    semaphore: Arc<Semaphore>,
    tasks: JoinSet<()>,
    shutdown: ShutdownToken,
}

impl WorkerPool {
    pub fn new(concurrency: usize, shutdown: ShutdownToken) -> Self {
        let semaphore = Arc::new(Semaphore::new(concurrency.max(1)));
        Self {
            semaphore,
            tasks: JoinSet::new(),
            shutdown,
        }
    }

    /// Spawn `fut` onto the pool, waiting for a permit. Returns immediately
    /// if the pool is already shutting down.
    pub async fn spawn<F>(&mut self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        if self.shutdown.is_cancelled() {
            return;
        }
        let permit = match self.semaphore.clone().acquire_owned().await {
            Ok(p) => p,
            Err(_) => return,
        };
        self.tasks.spawn(async move {
            let _permit = permit;
            fut.await;
        });
    }

    pub async fn drain(mut self) {
        while self.tasks.join_next().await.is_some() {}
    }
}
