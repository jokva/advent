use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

//#[macro_use]
extern crate itertools;

fn p1(mut order: HashMap<char, Vec<char>>) -> String {
    let mut steps = String::new();
    while !order.is_empty() {
        let mut ready = order.iter()
                             .filter(|(_, y)| y.is_empty())
                             .map(|(x, _)| x)
                             .map(|x| x.clone())
                             .collect::<Vec<char>>()
                             ;
        ready.sort();
        ready.dedup();
        ready.reverse();

        let current = ready.pop().unwrap();
        steps.push(current);

        /* now purge current from all dependency lists */
        for dep in order.values_mut() {
            dep.retain(|&x| x != current);
        }

        order.remove(&current);
    }

    steps
}

#[derive(Clone, Copy)]
struct Elf {
    time: i32,
    job: char
}

fn is_free(elf: &Elf) -> bool {
    elf.job == ' '
}

fn finished(elf: &Elf) -> bool {
    elf.time == 0
}

fn p2(mut order: HashMap<char, Vec<char>>) -> i32 {
    let mut time = 0;
    let mut elves = vec![Elf { time: 0, job: ' ' }; 5];

    let mut scheduled = vec![];

    while !order.is_empty() {
        /* finish jobs */
        for (i, mut elf) in elves.iter_mut().enumerate() {
            println!("{}: elf {} working on {}", time, i, elf.job);
            if finished(&elf) && !is_free(&elf) {
                scheduled.retain(|x| *x != elf.job);
                for dep in order.values_mut() {
                    dep.retain(|x| *x != elf.job);
                }
                elf.job = ' ';
            }
        }

        /* find new jobs */
        let mut ready = order.iter()
                             .filter(|(_, y)| y.is_empty())
                             .map(|(x, _)| x)
                             .map(|x| x.clone())
                             .collect::<Vec<char>>()
                             ;
        ready.sort();
        ready.dedup();
        ready.retain(|x| !scheduled.contains(x));

        for (mut elf, &job) in itertools::zip(
                            elves.iter_mut().filter(|x| is_free(x)),
                            &ready)
        {
            elf.job = job;
            let duration = (job as u32) - ('A' as u32) + 61;
            elf.time = duration as i32;
            scheduled.push(job);
            order.remove(&job);
            println!("Scheduling {}: {}s", job, duration);
        }

        elves.iter_mut()
            .filter(|x| !is_free(x))
            .map(|x| x.time -= 1)
            .collect::<()>()
            ;
        time += 1;
    }


    time + elves.iter().map(|x| x.time).max().unwrap()
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock()
                     .lines()
                     .map(|s| s.unwrap())
                     .map(|s| {
                        let mut pieces = s.split_whitespace();
                        let fst = pieces.nth(1).unwrap().chars().next().unwrap();
                        let snd = pieces.nth(5).unwrap().chars().next().unwrap();
                        (fst, snd)
                      })
                     .collect::<Vec<(char, char)>>()
                     ;

    let mut order = HashMap::<char, Vec<char>>::new();
    for (x, y) in input {
        order.entry(x).or_insert(vec![]);
        order.entry(y).or_insert(vec![]).push(x);
    }

    let steps = p1(order.clone());
    let time = p2(order.clone());
    println!("{}", steps);
    println!("{}", time);
}
