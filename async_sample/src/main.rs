use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::sync::mpsc::error::SendError;

struct WorkerPool {
    tx: mpsc::Sender<String>,
    rx: Arc<Mutex<mpsc::Receiver<String>>>,
}

impl WorkerPool {
    async fn new() -> Self {
        let (tx, rx) = mpsc::channel::<String>(32);
        WorkerPool {
            tx,
            rx: Arc::new(Mutex::new(rx))
        }
    }

    async fn push(&self, message: String) -> Result<(), SendError<String>> {
        self.tx.send(message).await
    }

    async fn pop(&self) -> Option<String> {
        self.rx.lock().await.recv().await
    }
}

async fn say_hello() {
    println!("Hello, world!");
}

#[tokio::main]
async fn main() {
    say_hello().await;

    let pool = WorkerPool::new().await;

    // Push with error handling
    match pool.push("Hello from async!".to_string()).await {
        Ok(()) => println!("Message pushed successfully"),
        Err(e) => eprintln!("Failed to push message: {:?}", e),
    }

    // Pop with error handling
    match pool.pop().await {
        Some(message) => println!("Received: {}", message),
        None => eprintln!("No message received (channel closed)"),
    }
}
