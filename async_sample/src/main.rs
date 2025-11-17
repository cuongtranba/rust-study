use async_channel::{bounded, Receiver, Sender};
use std::sync::Arc;
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

    // Spawn 3 workers sharing the same receiver
    let mut worker_handles = vec![];
    for worker_id in 1..=3 {
        let rx = rx.clone(); //
        let handle = tokio::spawn(worker(worker_id, rx));
        worker_handles.push(handle);
    }

    // Spawn producers
    let mut producer_handles = vec![];
    for i in 1..=10 {
        let pool_clone = Arc::clone(&pool);
        let handle = tokio::spawn(async move {
            pool_clone.submit(format!("Task {}", i)).await.unwrap();
        });
        producer_handles.push(handle);
    }

    // Wait for producers
    for handle in producer_handles {
        handle.await.unwrap();
    }

    drop(pool);
    drop(rx);

    for handle in worker_handles {
        handle.await.unwrap();
    }

    println!("All done!");
}
