use flume::{unbounded, Receiver};
use tokio::signal;
use std::time::Duration;

#[tokio::main]
pub async fn run() {
    let (tx, rx) = unbounded();

    for i in 0..4 {
        let worker_rx = rx.clone();
        tokio::spawn(async move {
            worker_loop(i, worker_rx).await;
        });
    }

    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    println!("Ctrl+C detected. Shutting down...");
    drop(tx);

    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("All workers stopped.");
}

async fn worker_loop(id: usize, rx: Receiver<()>) {
    loop {
        tokio::select! {
            _ = rx.recv_async() => {
                println!("Worker {} shutting down.", id);
                break;
            },
            _ = tokio::time::sleep(Duration::from_secs(1)) => {
                println!("Worker {} is working...", id);
            }
        }
    }
}
