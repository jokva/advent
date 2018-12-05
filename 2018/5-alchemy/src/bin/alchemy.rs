use std::io;
use std::io::prelude::*;

extern crate itertools;
use itertools::Itertools;

fn collapse(xs: &Vec<char>) -> Vec<char> {
    let len = xs.len();
    let mut ret = Vec::<char>::new();

    for (i, j) in itertools::zip(0.., 1..len) {
        let c = xs[i];
        let d = xs[j];

        if c != d && c.to_ascii_lowercase() == d.to_ascii_lowercase() {
            ret.extend_from_slice(&xs[j+1..]);
            return ret;
        } else {
            ret.push(c);
        }
    }

    ret.push(*xs.last().unwrap());
    ret
}

fn reduce_polymer(polymer: Vec<char>) -> Vec<char> {
    itertools::iterate(polymer, collapse)
               .tuple_windows()
               .skip_while(|(x, y)| x != y)
               .next()
               .unwrap()
               .0
}

fn main() {
    let stdin = io::stdin();
    let polymer: Vec<char> = stdin.lock().lines()
                                         .next()
                                         .unwrap()
                                         .map(|s| s.chars().collect())
                                         .unwrap();

    let formula = reduce_polymer(polymer.clone());
    println!("Formula length: {}", formula.len());

    let alphabet = (0..26).map(|x| (x + 'a' as u8) as char);

    let minformula = 
    alphabet.map(|c| polymer.clone().into_iter().filter(|x| x.to_ascii_lowercase() != c).collect())
            .map(reduce_polymer)
            .map(|x| x.len())
            .min()
            .unwrap()
            ;

    println!("Minimal length: {}", minformula);
}
