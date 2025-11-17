use async_channel::{bounded, Receiver, Sender};
use std::sync::Arc;
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

struct WorkerPool {
    tx: Sender<String>,
}

impl WorkerPool {
    fn new(capacity: usize) -> (Self, Receiver<String>) {
        let (tx, rx) = bounded::<String>(capacity);
        (WorkerPool { tx }, rx)
    }

    async fn submit(&self, message: String) -> Result<(), async_channel::SendError<String>> {
        self.tx.send(message).await
    }
}

async fn worker(id: usize, rx: Receiver<String>) {
    while let Ok(message) = rx.recv().await {
        println!("[Worker {}] Processing: {}", id, message);
        sleep(Duration::from_millis(200)).await;
        println!("[Worker {}] Done: {}", id, message);
    }
    println!("[Worker {}] Shutting down", id);
}

#[tokio::main]
async fn main() {
    let (pool, rx) = WorkerPool::new(32);
    let pool = Arc::new(pool);
    let mut worker_set = JoinSet::new();
    let mut producer_set = JoinSet::new();

    // Spawn 3 workers sharing the same receiver
    for worker_id in 1..=3 {
        let rx = rx.clone();
        worker_set.spawn(async move {
            worker(worker_id, rx).await;
        });
    }

    // Spawn producers
    for i in 1..=10 {
        let pool_clone = Arc::clone(&pool);
        producer_set.spawn(async move {
            pool_clone.submit(format!("Task {}", i)).await.unwrap();
        });
    }

    // Wait for all producers to finish
    while let Some(_) = producer_set.join_next().await {}

    // Drop the pool and rx to signal workers to stop
    drop(pool);
    drop(rx);

    // Wait for all workers to finish
    while let Some(_) = worker_set.join_next().await {}

    println!("All done!");
}
