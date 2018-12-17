use std::io;
use std::io::prelude::*;

extern crate itertools;
extern crate regex;

type Reservoir = Vec<Vec<char>>;

fn build_model(input: Vec<String>) -> (Reservoir, usize, usize) {
    let expr = r"(\w)=(\d+), \w=(\d+)\.\.(\d+)";
    let re = regex::Regex::new(expr).unwrap();

    let clay = input.iter().map(|line| {
        let bits = re.captures(&line).unwrap();
        let l = &bits[1];
        let lhs:  usize = bits[2].parse().unwrap();
        let rhs1: usize = bits[3].parse().unwrap();
        let rhs2: usize = bits[4].parse().unwrap();

        let xs = if l == "x" { lhs .. lhs + 1 } else { rhs1 .. rhs2 + 1 };
        let ys = if l == "y" { lhs .. lhs + 1 } else { rhs1 .. rhs2 + 1 };
        (xs, ys)
    }).collect::<Vec<_>>();

    let maxx = clay.iter()
                   .map(|x| x.clone())
                   .map(|x| x.0.max().unwrap())
                   .max()
                   .unwrap()
                   + 1 // account for zero-indexing
                   + 1 // add halo for right-hand-side, so stuff can flow
                   ;
    let maxy = clay.iter()
                   .map(|x| x.clone())
                   .map(|x| x.1.max().unwrap())
                   .max()
                   .unwrap()
                   + 1
                   ;

    let minx = clay.iter()
                   .map(|x| x.clone())
                   .map(|x| x.0.min().unwrap())
                   .min()
                   .unwrap()
                   - 1
                   ;

    let miny = clay.iter()
                   .map(|x| x.clone())
                   .map(|x| x.1.min().unwrap())
                   .min()
                   .unwrap()
                   ;

    let mut reservoir = vec![vec!['.'; maxx - minx]; maxy - miny];

    for (xs, ys) in clay {
        for x in xs {
            for y in ys.clone() {
                reservoir[y - miny][x - minx] = '#';
            }
        }
    }

    (reservoir, minx, miny)
}

fn drip(mut res: Reservoir,
        srcx: usize,
        srcy: usize)
    -> Reservoir {

    if srcy == res.len() { return res }

    /* clay, or already set */
    if res[srcy][srcx] == '#' { return res }
    if res[srcy][srcx] == '|' { return res }
    if res[srcy][srcx] == '~' { return res }

    /* water goes through here */
    if res[srcy][srcx] == '.' {
        res[srcy][srcx] = '|';
    }

    let nexty = srcy + 1;
    /* flow down first */
    res = drip(res, srcx, nexty);

    /* at bottom - exit */
    if nexty == res.len() { return res }

    match res[nexty][srcx] {
        '.' | '|' => (),
        '#' => {
            /* hit clay, flow to the sides */
            res = drip(res, srcx-1, srcy);
            res = drip(res, srcx+1, srcy);

         },
        '~' => {
            if (nexty .. res.len())
                .map(|y| res[y][srcx])
                .filter(|&c| c == '#')
                .next()
                .is_some() {
                    /* 
                     * water is pushing back, and there is clay directy below
                     */
                    res = drip(res, srcx-1, srcy);
                    res = drip(res, srcx+1, srcy);
            };
         },
         x  => panic!("Unknown cell {}", x),
    }

    /* 
     * if everything in this row is horizontal water (|), and between two walls
     * of clay, then change them all to ~
     */
    let left = (0 .. srcx)
                .rev()
                .map(|i| res[srcy][i])
                .take_while(|&c| c != '#')
                .collect::<Vec<_>>()
                ;

    if left.len() == srcx {
        /* 
         * wasn't even a # there, so stream is pushed all the way to left edge
         * before pouring down, and should not be filled
         */
        return res
    }

    let right = (srcx .. res[0].len())
                    .map(|i| res[srcy][i])
                    .take_while(|&c| c != '#')
                    .collect::<Vec<_>>()
                    ;

    if right.len() == res[0].len() - (srcx + 1) {
        return res
    }

    if left.iter().all(|&c| c == '|') && right.iter().all(|&c| c == '|') {
        for i in (0 .. srcx).rev() {
            if res[srcy][i] == '#' { break }
            res[srcy][i] = '~';
        }

        for i in srcx .. res[0].len() {
            if res[srcy][i] == '#' { break }
            res[srcy][i] = '~';
        }
    }

    res
}

fn print(reservoir: &Reservoir) -> () {
    for ln in reservoir.iter() {
        println!("{}", ln.iter().collect::<String>());
    }
    println!("");
}

fn main() {
    let stdin = io::stdin();
    let input =
    stdin.lock()
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        ;

    let (model, offset, cut) = build_model(input);

    let step = drip(model, 500 - offset, 0);
    print(&step);
    let pooled_water = step.iter()
                    .flat_map(|x| x)
                    .filter(|&&c| c == '~')
                    .count()
                    ;

    let water = step.iter()
                    .flat_map(|x| x)
                    .filter(|&&c| c == '|')
                    .count()
                    + pooled_water
                    ;


    println!("Water rests in {} cells, can reach {}", pooled_water, water);
}
