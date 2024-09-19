use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

fn start_workers(num_workers: usize, rx: Arc<Mutex<Receiver<Option<i32>>>>) {
    for id in 0..num_workers {
        let rx = Arc::clone(&rx);
        thread::spawn(move || loop {
            let data = {
                let rx = rx.lock().unwrap();
                rx.recv().unwrap_or(None)
            };

            match data {
                Some(value) => {
                    println!("Worker {} received: {}", id, value);
                    thread::sleep(Duration::from_millis(50));
                }
                None => {
                    println!("Worker {}: no more data.", id);
                    break;
                }
            }
        });
    }
}

pub fn run() {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let num_workers = 4;
    start_workers(num_workers, Arc::clone(&rx));

    for i in 1..=10 {
        tx.send(Some(i)).unwrap();
        println!("Sent: {}", i);
        thread::sleep(Duration::from_millis(100));
    }

    for _ in 0..num_workers {
        tx.send(None).unwrap();
    }

    println!("All data sent.");
}
