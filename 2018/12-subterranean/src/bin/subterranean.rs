use std::io;
use std::io::prelude::*;

type Pattern = (Vec<i32>, i32);

extern crate itertools;

fn str_to_pattern(s: &str, from: usize, to: usize) -> Vec<i32> {
    s.chars()
     .skip(from)
     .take(to)
     .map(|c| if c == '.' { 0 } else { 1 })
     .collect()
}

fn evolve(pots: &Vec<i32>, patterns: &Vec<Pattern>) -> Vec<i32> {
    let asstr: String = pots.iter().map(|c| if *c == 0 { '.' } else { '#' }).collect();
    let begin = pots[..2].iter().map(|x| *x);
    let end = pots[pots.len() - 2 .. pots.len()].iter().map(|x| *x);
    let windows = pots[..].windows(5).map(|window| {
        let win = window.to_vec();
        match patterns.iter().find(|(x, _)| *x == win) {
            Some(x) => x.1,
            None    => 0,
        }
    });

    begin.chain(windows).chain(end).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let initial: Vec<i32> = lines.next()
                                  .unwrap()
                                  .map(|s| str_to_pattern(&s, 15, s.len()))
                                  .unwrap()
                                  ;
    // skip the empty line
    lines.next();

    let patterns = lines.map(|s| s.unwrap())
                        .map(|s| {
                            let pat = str_to_pattern(&s, 0, 5);
                            let res = if s.chars().last().unwrap()
                                        == '.' { 0 }
                                        else   { 1 };
                            (pat, res)
                        })
                        .collect::<Vec<Pattern>>()
                        ;

    let zero = 5000;
    let mut pots = vec![0; 5 * initial.len() + zero];

    for i in 0 .. initial.len() {
        pots[zero + i] = initial[i];
    }

    let lastgen = itertools::iterate(pots.clone(), |x| evolve(&x, &patterns))
                        .skip(20).next().unwrap();

    let plants: i32 = (0..).zip(lastgen.iter())
                           .filter(|(_, &x)| x == 1)
                           .map(|(i, _)| (i as i32 - zero as i32))
                           .sum();

    println!("Plants: {}", plants);

    let k10: i64 = 200;
    let genx = itertools::iterate(pots, |x| evolve(x, &patterns))
                        .skip(k10 as usize)
                        .next()
                        .unwrap()
                        ;

    let bn50 = 50000000000;
    let plantsx: i64 = (0..).zip(genx.iter())
                            .filter(|(_, &x)| x == 1)
                            .map(|(i, _)| bn50 - k10 + (i as i64 - zero as i64))
                            .sum::<i64>();

    println!("Plants 50bn: {}", plantsx);

}
