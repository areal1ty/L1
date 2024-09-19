use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

async fn producer(mut tx: mpsc::Sender<i32>) {
    let mut value = 0;
    loop {
        if tx.send(value).await.is_err() {
            println!("Receiver dropped");
            return;
        }
        println!("Sent: {}", value);
        value += 1;
        sleep(Duration::from_secs(1)).await;
    }
}

async fn consumer(mut rx: mpsc::Receiver<i32>) {
    while let Some(value) = rx.recv().await {
        println!("Received: {}", value);
    }
}

#[tokio::main]
pub async fn run() {
    let (tx, rx) = mpsc::channel(10);
    let n_seconds = 5;

    tokio::spawn(producer(tx));
    tokio::spawn(consumer(rx));

    sleep(Duration::from_secs(n_seconds)).await;

    println!("Shutting down after {} seconds.", n_seconds);
}