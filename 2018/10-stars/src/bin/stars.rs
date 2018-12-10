use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;

extern crate regex;

#[derive(Copy, Clone, Eq)]
struct Point {
    x: i32,
    y: i32,
    xv: i32,
    yv: i32,
}

impl Point {
    fn play(&mut self) -> &mut Self {
        self.x += self.xv;
        self.y += self.yv;
        self
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}) -> [{}, {}]", self.x, self.y, self.xv, self.yv)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.x == other.x {
            self.y.cmp(&other.y)
        } else {
            self.x.cmp(&other.x)
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn main() {
    let re = regex::Regex::new(
        "position=<([^,]+),([^>]+)> velocity=<([^,]+),([^>]+)>"
    ).unwrap();
    let stdin = io::stdin();
    let mut points = stdin.lock()
                      .lines()
                      .map(|s| s.unwrap())
                      .map(|s| {
                          let cap = re.captures(&s).unwrap();
                          let x  = cap[1].trim().parse().unwrap();
                          let y  = cap[2].trim().parse().unwrap();
                          let xv = cap[3].trim().parse().unwrap();
                          let yv = cap[4].trim().parse().unwrap();
                          Point { x: x, y: y, xv: xv, yv: yv }
                      }).collect::<Vec<Point>>()
                      ;

    let mut time = 0;
    loop {
        points.sort();
        let min = points.first().unwrap().clone();
        let max = points.last().unwrap().clone();

        // this heuristic is pretty specific to full problem size input
        // but can probably be computed from the number of points
        if max.x - min.x < 70 {
            /* correct all points by making them [0, ..) */
            let minx = points.iter().map(|x| x.x).min().unwrap();
            let miny = points.iter().map(|x| x.y).min().unwrap();
            for mut p in points.iter_mut() {
                p.x += -minx;
                p.y += -miny;
            }

            let left  = points.iter().map(|x| x.x).min().unwrap();
            let right = points.iter().map(|x| x.x).max().unwrap();
            let top   = points.iter().map(|x| x.y).min().unwrap();
            let bot   = points.iter().map(|x| x.y).max().unwrap();


            let xs = 1 + right - left;
            let ys = 1 + bot - top;
            let mut canvas = vec![' '; (xs * ys) as usize];

            for p in &points {
                let x = p.x;
                let y = p.y;
                canvas[(x + y*xs) as usize] = '#';
            }

            for y in 0 .. ys {
                let fst = (y * xs) as usize;
                let lst = ((y+1) * xs) as usize;
                let ln: String = canvas[fst .. lst].iter().collect();
                println!("{}", ln);
            }
            break;
        }

        for mut p in points.iter_mut() {
            p.play();
        }

        time += 1;
    }

    println!("The elves had to wait for {} seconds", time);
}
