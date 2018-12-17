use std::io;
use std::io::prelude::*;

extern crate itertools;
extern crate regex;

type Reservoir = Vec<Vec<char>>;

fn build_model(input: Vec<String>) -> (Reservoir, usize) {
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

    let mut reservoir = vec![vec!['.'; maxx - minx]; maxy];

    for (xs, ys) in clay {
        for x in xs {
            for y in ys.clone() {
                reservoir[y][x - minx] = '#';
            }
        }
    }

    reservoir[0][500 - minx] = '+';
    (reservoir, minx)
}

fn drip(mut res: Reservoir,
        srcx: usize,
        srcy: usize)
    -> Reservoir {

    print(&res);
    if srcy == res.len()      { return res }
    if res[srcy][srcx] == '#' { return res }
    if res[srcy][srcx] == '|' { return res }

    res[srcy][srcx] = '|';

    /* flow down first */
    let nexty = srcy + 1;
    res = drip(res, srcx, nexty);

    /* at bottom - exit */
    if nexty == res.len() { return res }

    match res[nexty][srcx] {
        '.' => (),
        '#' => {
            /* hit clay, spray to the sides */
            res = drip(res, srcx-1, srcy);
            res = drip(res, srcx+1, srcy);
         },
        '|' => {
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

    let (model, offset) = build_model(input);

    let step = drip(model, 500 - offset, 1);
}
