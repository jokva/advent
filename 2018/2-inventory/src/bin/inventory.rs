use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

#[macro_use] extern crate itertools;

type Chars = Vec<char>;

fn occurences(chars: &Chars) -> HashMap<char, i32>
{
    let mut counts = HashMap::<char, i32>::new();
    for c in chars {
        *counts.entry(*c).or_insert(0) += 1;
    }

    counts
}

fn gather<'a, I>(vals: I, len: i32) -> Vec<Chars>
where
     I: Iterator<Item = &'a Chars>
{
    let mut strings = vec![];
    for ref string in vals {
        match occurences(&string).values().find(|&&x| x == len) {
            Some(_) => strings.push(string.to_vec()),
            _       => ()
        }
    }

    strings
}

fn difference(xs: &Chars, ys: &Chars) -> i32 {
    xs.iter().zip(ys).map(|(x, y)| if x == y { 0 } else { 1 }).sum()
}

fn rmdup(xs: &Chars, ys: &Chars) -> String {
    xs.iter()
        .zip(ys)
        .filter(|(&x, &y)| x == y)
        .map(|(x, _)| x)
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let lns: Vec<Chars> = stdin.lock()
                            .lines()
                            .map(|x| x.unwrap().chars().collect())
                            .collect()
                            ;

    let vec2 = gather(lns.iter(), 2);
    let vec3 = gather(lns.iter(), 3);
    println!("checksum: {}", vec2.len() * vec3.len());

    let cat = [&vec2[..], &vec3[..]].concat();
    for (xs, ys) in iproduct!(&cat, &cat) {
        if difference(&xs, &ys) == 1 {
            println!("box: {}", rmdup(xs, ys));
            break;
        }
    }
}
