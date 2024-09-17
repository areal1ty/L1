mod l1;
mod l2;
mod l3;

use crate::l1::task1;
use crate::l2::task2;
use crate::l3::task3;

fn main() {
    task1::run();
    task2::run(10);
    task3::run(10)
}
