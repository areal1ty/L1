use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    let cloned_token = token.clone();

    let task = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cloned_token.cancelled() => {
                    println!("Task was cancelled");
                    break;
                }
                _ = sleep(Duration::from_secs(1)) => {
                    println!("Task is running");
                }
            }
        }
    });

    sleep(Duration::from_secs(3)).await;
    token.cancel();
    task.await.unwrap();
}
