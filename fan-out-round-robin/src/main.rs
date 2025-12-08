use flume;

#[tokio::main]
async fn main() {
    let (tx, rx) = flume::bounded::<i32>(100);

    for worker in 0..3 {
        let rx = rx.clone();
        tokio::spawn(async move {
            while let Ok(task) = rx.recv_async().await {
                println!("Worker {} got {}", worker, task);
            }
        });
    }

    for task in 0..10 {
        tx.send_async(task).await.unwrap();
    }
    drop(tx);
}
