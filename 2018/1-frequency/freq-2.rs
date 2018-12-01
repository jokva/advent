use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let currents: Vec<i32> = stdin.lock().lines().map(|s|
        s.unwrap().parse::<i32>().unwrap()
    ).collect();

    let mut seen = HashSet::new();
    let mut total: i32 = 0;
    seen.insert(total);

    for current in currents.iter().cycle() {
        total += current;
        if seen.contains(&total) { break; }
        seen.insert(total);
    }

    println!("{}", total)
}
