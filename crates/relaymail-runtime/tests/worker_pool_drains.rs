use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use relaymail_runtime::{ShutdownToken, WorkerPool};

#[tokio::test]
async fn pool_executes_and_drains() {
    let shutdown = ShutdownToken::new();
    let mut pool = WorkerPool::new(4, shutdown);
    let counter = Arc::new(AtomicUsize::new(0));
    for _ in 0..10 {
        let c = counter.clone();
        pool.spawn(async move {
            tokio::time::sleep(Duration::from_millis(1)).await;
            c.fetch_add(1, Ordering::SeqCst);
        })
        .await;
    }
    pool.drain().await;
    assert_eq!(counter.load(Ordering::SeqCst), 10);
}

#[tokio::test]
async fn pool_refuses_new_spawns_after_cancel() {
    let shutdown = ShutdownToken::new();
    shutdown.cancel();
    let mut pool = WorkerPool::new(2, shutdown);
    let counter = Arc::new(AtomicUsize::new(0));
    let c = counter.clone();
    pool.spawn(async move {
        c.fetch_add(1, Ordering::SeqCst);
    })
    .await;
    pool.drain().await;
    assert_eq!(counter.load(Ordering::SeqCst), 0, "spawn was skipped");
}
