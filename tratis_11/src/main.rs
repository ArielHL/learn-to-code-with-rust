#![allow(unused)]
use std::{ops::Add, process::Output};

fn add_two_numbers<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let a = 10.3;
    let b = 20.9;
    let c = add_two_numbers(a, b);
    println!("c: {}", c);
}
