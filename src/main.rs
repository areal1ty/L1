mod l1;
mod l2;
mod l3;
mod l4;
mod l5;
mod l24;
mod l22;
mod l23;

use crate::l1::task1;
use crate::l22::task22;
use crate::l24::task24;
use crate::l2::task2;
use crate::l3::task3;
use crate::l4::task4;
use crate::l5::task5;

fn main() {
    //task1::run();
    //task2::run(10);
    //task3::run(10);
    // task4::run();
    // task5::run();
    // println!("{}", task24::run("abcd"));
    let mut v = vec![10, 20, 30, 40, 50];
    println!("Vector without deleted element {:?} is {:?}", task22::remove_element_at(&mut v, 2), v);
}
