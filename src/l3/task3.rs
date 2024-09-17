use std::thread;

pub fn run(n: u32) {
    let numbers: Vec<u32> = (1..=n).collect();

    numbers.into_iter()
        .map(|num| {
            thread::spawn(move || {
                println!("{}^2 + {}^2 = {}", num, num, (num*num) + (num*num))
            })
        })
        .for_each(|handle| handle.join().unwrap());
}
