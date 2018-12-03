use std::io;
use std::io::prelude::*;

struct Quilt {
    id: i32,
    fromleft: i32,
    fromtop: i32,
    width: i32,
    height: i32,
}

fn parse(ln: String) -> Quilt {
    let words: Vec<&str> = ln.split(' ').collect();
    let id: i32 = words[0][1..].parse().unwrap();
    let mut pos = words[2].split(',');
    let fromleft: i32 = pos.next().unwrap().parse().unwrap();
    let fromtop: i32 = pos.next().unwrap().trim_matches(':').parse().unwrap();
    let mut size = words[3].split('x');
    let width: i32  = size.next().unwrap().parse().unwrap();
    let height: i32 = size.next().unwrap().parse().unwrap();
    Quilt {
        id: id,
        fromleft: fromleft,
        fromtop: fromtop,
        width: width,
        height: height,
    }
}

fn unique_quilt(quilts: &Vec<Quilt>, board: &Vec<i32>, cols: i32) -> i32 {
    'outer: for quilt in quilts {
        let left = quilt.fromleft;
        let top  = quilt.fromtop;

        let right = left + quilt.width;
        let bot = top + quilt.height;

        for i in left..right {
            for j in top..bot {
                if board[(i + cols * j) as usize] != 1 {
                    continue 'outer;
                }
            }
        }

        return quilt.id;
    }

    panic!("no unique overlap");
}

fn main() {
    let stdin = io::stdin();
    let quilts: Vec<Quilt> = stdin.lock()
                                .lines()
                                .map(|s| s.unwrap())
                                .map(parse)
                                .collect()
                                ;

    // The board is only 1000x1000, so just populate an explicit array
    // technically it says least 1000x1000, but it seems not to apply
    // otherwise, consider a quad tree or similar
    let cols: i32 = 1000;
    let rows: i32 = 1000;
    let mut board = vec![0; (cols * rows) as usize];

    for quilt in &quilts {
        let left = quilt.fromleft;
        let top  = quilt.fromtop;

        let right = left + quilt.width;
        let bot = top + quilt.height;

        for i in left..right {
            for j in top..bot {
                board[(i + cols * j) as usize] += 1;
            }
        }
    }

    let overlapped: i32 = board.iter().map(|x| if x > &1 { 1 } else { 0 }).sum();
    let unique = unique_quilt( &quilts, &board, cols);

    println!("overlapping: {}", overlapped);
    println!("unique: {}", unique);
}
