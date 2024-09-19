use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            println!("Got: {}", msg);
        }
        println!("Thread stopping");
    });

    tx.send("Hello").unwrap();
    thread::sleep(Duration::from_secs(1));
    drop(tx);
    handle.join().unwrap();
}
