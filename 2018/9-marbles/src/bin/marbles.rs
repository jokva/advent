use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::mem;
use std::cmp;

fn marbles(nplayers: usize, last: usize) -> usize {
    let mut players = vec![0; nplayers];
    let mut left = VecDeque::from(vec![0]);
    let mut right = VecDeque::from(vec![]);

    for marble in 1 .. last+1 {
        //println!("{:?} {:?}", left, right);
        if marble % 23 == 0 {
            let player = (marble-1) as usize % players.len();
            let skip = 8;
            let fromleft = cmp::min(skip, left.len());
            let fromright = skip - fromleft;
            for _ in 0 .. fromleft {
                right.push_front(left.pop_back().unwrap())
            }

            if fromright > 0 {
                mem::swap(&mut left, &mut right);
            }

            for _ in 0 .. fromright {
                right.push_front(left.pop_back().unwrap())
            }

            players[player] += right.pop_front().unwrap();
            players[player] += marble;
            left.push_back(right.pop_front().unwrap());
            continue;
        }

        if right.is_empty()  {
            mem::swap(&mut left, &mut right);
        }

        let r = right.pop_front();
        left.push_back(r.unwrap());
        left.push_back(marble);
    }

    *players.iter().max().unwrap()
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    let mut input = line.split(' ');
    let players = input.next().unwrap().parse::<usize>().unwrap();
    let last = input.nth(5).unwrap().parse::<usize>().unwrap();

    let p1 = marbles(players, last);
    let p2 = marbles(players, last * 100);
    println!("P1: {} points", p1);
    println!("P2: {} points", p2);
}
