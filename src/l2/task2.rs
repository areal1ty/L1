use std::thread;

pub fn run() {
    let numbers: Vec<u32> = (1..=10).collect();

    numbers.into_iter()
        .map(|num| {
            thread::spawn(move || {
                println!("{}^2 = {}", num, num * num);
            })
        })
        .for_each(|handle| handle.join().unwrap());
}
