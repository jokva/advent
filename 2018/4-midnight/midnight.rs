use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn sortedinput() -> Vec<String> {
    let stdin = io::stdin();
    let mut lines: Vec<String> = stdin.lock()
                                      .lines()
                                      .map(|x| x.unwrap())
                                      .collect()
                                      ;
    lines.sort();
    return lines;
}

fn main() {
    let lines = sortedinput();

    let mut naps = HashMap::<String, Vec<i32>>::new();
    let mut current: String = "error".to_string();

    for ln in lines {
        let onspace: Vec<&str> = ln.split("] ").collect();
        let timestamp: usize = onspace[0][1..].split(':')
                                              .last()
                                              .unwrap()
                                              .parse()
                                              .unwrap();
        match onspace[1].split(' ').collect::<Vec<&str>>()[1] {
            "up" => {
                naps.get_mut(&current)
                    .unwrap()[timestamp..]
                    .iter_mut()
                    .map(|x| *x -= 1)
                    .collect::<()>()
            },
            "asleep" => {
                naps.get_mut(&current)
                    .unwrap()[timestamp..]
                    .iter_mut()
                    .map(|x| *x += 1)
                    .collect::<()>()
            },
            x => {
                current = x.to_string();
                match naps.get(&current) {
                    None => naps.insert(current.clone(), vec![0; 60]),
                    _    => None
                };
                ()
            }
        }
    }

    let sleepyhead = naps.iter()
                     .map(|(guard, sleep)| (sleep.iter().sum::<i32>(), guard))
                     .max()
                     .map(|(_, guard)| guard)
                     .unwrap()
                     ;

    let (_, minute) = naps.get(sleepyhead)
                          .unwrap()
                          .iter()
                          .enumerate()
                          .map(|(i,x)| (x,i))
                          .max()
                          .unwrap()
                          ;

    let guard_id: usize = sleepyhead[1..].parse().unwrap();
    println!("{} at {}: {}", sleepyhead, minute, guard_id * minute);

    let asleep_at = naps.iter()
                        .map(|(guard, sleep)| (sleep.iter()
                                                    .enumerate()
                                                    .map(|(i,x)| (x,i))
                                                    .max()
                                                    .unwrap(),
                                               guard))
                        .max()
                        .map(|((_, minute), id)| (minute, id[1..]
                                                            .parse::<usize>()
                                                            .unwrap()))
                        .map(|(x,y)| x * y)
                        .unwrap()
                        ;

    println!("{}", asleep_at);
}
