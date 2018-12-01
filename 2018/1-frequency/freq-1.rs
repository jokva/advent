use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let currents: Vec<i32> = stdin.lock().lines().map(|s|
        s.unwrap().parse::<i32>().unwrap()
    ).collect();

    let total:i32 = currents.iter().sum();

    println!("{}", total)
}
