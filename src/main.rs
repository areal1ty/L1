mod l1;
mod l2;
mod l3;
mod l4;

use crate::l1::task1;
use crate::l2::task2;
use crate::l3::task3;
use crate::l4::task4;

fn main() {
    task1::run();
    task2::run(10);
    task3::run(10);
    task4::run();
}
