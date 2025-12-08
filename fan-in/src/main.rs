use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();
    for _worker in 0..4 {
        let tx = tx.clone();
        std::thread::spawn(move || {
            for i in 0..10 {
                tx.send(i).unwrap();
            }
        });
    }
    drop(tx);
    for received in rx {
        println!("Got: {}", received);
    }
    println!("Hello, world!");
}
