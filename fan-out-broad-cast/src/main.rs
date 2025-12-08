use std::time::Duration;

use tokio::sync::broadcast;
#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel::<String>(100);
    for worker_id in 0..3 {
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            while let Ok(value) = rx.recv().await {
                println!("Worker {} received {}", worker_id, value);
            }
        });
    }

    tokio::time::sleep(Duration::from_millis(10)).await;
    tx.send("Hello".to_string()).unwrap();
    tx.send("World".to_string()).unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
}
