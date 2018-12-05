use std::io;
use std::io::prelude::*;

extern crate itertools;
use itertools::Itertools;

fn collapse(xs: &Vec<char>) -> Vec<char> {
    let len = xs.len();
    let mut ret = Vec::<char>::new();
    let mut i = 0;
    let mut j = 1;

    while j < len {
        let c = xs[i];
        let d = xs[j];

        if c != d && c.to_ascii_lowercase() == d.to_ascii_lowercase() {
            // match
            i += 1;
            j += 1;
        } else {
            ret.push(c);
        }

        i += 1;
        j += 1;
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
    alphabet.map(|c| polymer.clone()
                            .into_iter()
                            .filter(|x| x.to_ascii_lowercase() != c)
                            .collect())
            .map(reduce_polymer)
            .map(|x| x.len())
            .min()
            .unwrap()
            ;

    println!("Minimal length: {}", minformula);
}
