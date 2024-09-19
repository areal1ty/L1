/*use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::{fs, sync::Arc};
use flume::{Receiver, Sender};
use tokio::signal;
use tokio::task;
use std::time::Duration;

#[tokio::main]
pub async fn run() {
    let (sender, receiver) = flume::unbounded();
    let worker_pool = WorkerPool::new(4, receiver);

    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    println!("Server running on 127.0.0.1:7878");

    let server_handle = tokio::spawn(async move {
        loop {
            match listener.accept() {
                Ok((stream, _)) => {
                    let sender_clone = sender.clone();
                    worker_pool.execute(async move || {
                        handle_connection(stream, sender_clone).await;
                    }).await;
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {:?}", e);
                    break;
                }
            }
        }
    });

    // Обработка Ctrl+C
    signal::ctrl_c().await.expect("Failed to listen for ctrl_c");

    println!("Shutting down gracefully...");
    worker_pool.shutdown().await;
    server_handle.await.unwrap();
}

async fn handle_connection(mut stream: tokio::net::TcpStream, _sender: Sender<Job>) {
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await.unwrap();

    let request = String::from_utf8_lossy(&buffer[..n]);
    let (status_line, filename) = if request.starts_with("GET / HTTP/1.1") {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if request.starts_with("GET /sleep HTTP/1.1") {
        tokio::time::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).await.unwrap();
}

struct WorkerPool {
    sender: Sender<Job>,
    receiver: Arc<Receiver<Job>>,
    workers: Vec<task::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl WorkerPool {
    pub fn new(size: usize, receiver: Receiver<Job>) -> WorkerPool {
        let receiver = Arc::new(receiver);
        let mut workers = Vec::with_capacity(size);
        let (sender, recv) = flume::unbounded();

        for _ in 0..size {
            let receiver_clone = Arc::clone(&recv);
            workers.push(task::spawn(async move {
                loop {
                    match receiver_clone.recv_async().await {
                        Ok(job) => {
                            job();
                        }
                        Err(_) => {
                            break;
                        }
                    }
                }
            }));
        }

        WorkerPool { sender, receiver, workers }
    }

    pub async fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send_async(job).await.unwrap();
    }

    pub async fn shutdown(self) {
        drop(self.sender);
        for worker in self.workers {
            worker.await.unwrap();
        }
        println!("All workers shut down.");
    }
}


 */