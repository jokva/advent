use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

#[macro_use] extern crate itertools;

struct Coordinate {
    id: i32,
    x: i32,
    y: i32 
}

fn boardsize(coords: &Vec<Coordinate>) -> (i32, i32) {
    let x = coords.iter().map(|x| x.x).max().unwrap();
    let y = coords.iter().map(|x| x.y).max().unwrap();

    (x + 1, y + 1)
}

fn distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn closest(points: &Vec<Coordinate>, x: i32, y: i32) -> i32 {
    let mut v: Vec<(i32, &Coordinate)> = points
                                    .iter()
                                    .map(|c| (distance((c.x, c.y), (x, y)), c))
                                    .collect()
                                    ;
    v.sort_by(|(x,_), (y,_)| x.cmp(y));
    let max = v.first().unwrap();

    match v.iter().take_while(|c| c.0 == max.0).count() {
        1 => { max.1.id },
        _ => -1
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock()
                     .lines()
                     .map(|x| x.unwrap())
                     .collect::<Vec<String>>();
    let points = input.iter()
                      .map(|s| s.split(", "))
                      .enumerate()
                      .map(|(id, mut x)| Coordinate {
                          id: id as i32,
                          x: x.next().unwrap().parse().unwrap(),
                          y: x.next().unwrap().parse().unwrap()
                        })
                      .collect::<Vec<Coordinate>>();

    let (xs, ys) = boardsize(&points);

    let mut dists = vec![0; (xs * ys) as usize];

    for (x, y) in iproduct!(0..xs, 0..ys) {
        dists[(x + y * xs) as usize] = closest(&points, x, y);
    }

    for x in 0..xs {
        let v = dists[x as usize];
        if v != -1 {
            dists.iter_mut()
                 .map(|e| *e = if *e == v { -1 } else { *e })
                 .collect::<()>();
        }
    }

    for x in 0..xs {
        let v = dists[(x + xs * (ys - 1))  as usize];
        if v != -1 {
            dists.iter_mut()
                 .map(|e| *e = if *e == v { -1 } else { *e })
                 .collect::<()>();
        }
    }

    for y in 0..ys {
        let v = dists[(y * xs) as usize];
        if v != -1 {
            dists.iter_mut()
                 .map(|e| *e = if *e == v { -1 } else { *e })
                 .collect::<()>();
        }
    }

    for y in 0..ys {
        let v = dists[((y + 1) * ys - 1) as usize];
        if v != -1 {
            dists.iter_mut()
                 .map(|e| *e = if *e == v { -1 } else { *e })
                 .collect::<()>();
        }
    }

    let mut counts = HashMap::<i32, i32>::new();
    for id in dists.iter().filter(|&&x| x != -1) {
        *counts.entry(*id).or_insert(0) += 1;
    }

    let largest = counts.iter()
                    .max_by(|(_, y), (_, b)| y.cmp(b))
                    .unwrap();
    println!("Largest area {}, id {}", largest.1, largest.0);

    let mut area: i32 = 0;
    for (x, y) in iproduct!(-500..5000, -500..5000) {
        if points.iter()
                 .map(|p| distance((p.x, p.y), (x, y)))
                 .sum::<i32>() < 10000 {
            area += 1;
        }
    }
    println!("Largest region: {}", area);
}
