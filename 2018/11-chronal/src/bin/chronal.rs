use std::io;
use std::io::prelude::*;

#[macro_use] extern crate itertools;

fn rack_ids(dims: i32) -> Vec<i32> {
    std::iter::repeat(11..dims+11).take(300)
               .flat_map(|x| x)
               .collect()
}

fn third_digit(xs: String) -> i32 {
    let x: Vec<i32> = xs.chars()
                        .map(|d| (d as u32 - '0' as u32) as i32)
                        .collect();
    x[(x.len() - 3 as usize)]
}

fn max_nxn(grid: &Vec<i32>, dims: i32, n: i32) -> (i32, (i32, i32)) {
    let xs = 0..dims-n;
    let ys = 0..dims-n;

    iproduct!(xs, ys).map(|(x, y)|
        (iproduct!(x..x+n, y..y+n)
            .map(|(i, j)| grid[(i + j * dims) as usize])
            .sum(),
        (x+1, y+1))
    ).max().unwrap()
}

fn main() {
    let stdin = io::stdin();
    let serial: i32 = stdin.lock()
                           .lines()
                           .next()
                           .unwrap()
                           .unwrap()
                           .parse()
                           .unwrap();
    println!("Serial: {}", serial);

    let rack = rack_ids(300);
    let cells = rack.iter()
                    .zip((1..301).map(|x| std::iter::repeat(x).take(300))
                                    .flat_map(|x| x))
                    .map(|(y, x)| x * y)
                    .map(|x| x + serial)
                    .zip(rack.iter())
                    .map(|(x, y)| x * y)
                    .map(|x| x.to_string())
                    .map(|x| third_digit(x))
                    .map(|x| x - 5)
                    .collect::<Vec<i32>>()
                    ;
    let (_, (x, y)) = max_nxn(&cells, 300, 3);
    println!("3: at ({},{})", x, y);

    let (_, xmax, ymax, n) = (1..300).map(|n| {
                                        println!("{}", n);
                                        let (pow, (x,y)) = max_nxn(&cells, 300, n);
                                        (pow, x,y,n)
                                      })
                                      .max()
                                      .unwrap()
                                      ;
    println!("x/y/size {},{},{}", xmax, ymax, n);
}
