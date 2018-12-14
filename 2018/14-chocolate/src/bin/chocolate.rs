use std::io;
use std::io::prelude::*;

fn toints(s: String) -> Vec<i32> {
    s.chars().map(|d| d as i32 - '0' as i32).collect()
}

fn mkrecipes(xs: &Vec<i32>, elf1: usize, elf2: usize) -> Vec<i32> {
    toints((xs[elf1] + xs[elf2]).to_string())
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock()
                     .lines()
                     .next()
                     .unwrap()
                     .unwrap()
                     .parse()
                     .unwrap()
                     ;

    let mut elf1 = 0;
    let mut elf2 = 1;
    let mut recipes = vec![3, 7];

    // how far to go determined by experiments
    for _ in 0 .. input + 100000000 {
        let new_recipes = mkrecipes(&recipes, elf1, elf2);
        recipes.extend(new_recipes);
        elf1 = (elf1 + 1 + recipes[elf1] as usize) % recipes.len();
        elf2 = (elf2 + 1 + recipes[elf2] as usize) % recipes.len();
    }

    let out: String = recipes[input..input+10]
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .concat()
                        ;
    println!("{}", out);

    let input_recipes = toints(input.to_string());
    let before_input_sequence =
    recipes[..].windows(input_recipes.len())
               .take_while(|x| !x.iter()
                                 .zip(input_recipes.iter())
                                 .all(|(a, b)| a == b))
               .count()
               ;
    println!("{}", before_input_sequence);
}
