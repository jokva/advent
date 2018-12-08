use std::io;
use std::io::prelude::*;

struct Tree {
    children: Vec<Tree>,
    meta: Vec<i32>,
}

fn tree(source: &Vec<i32>, at: usize) -> (Tree, usize) {
    if source.len() <= at {
        return ( Tree { children: vec![], meta: vec![] }, 0 )
    }

    let nchildren = source[at];
    let nmetas = source[at+1];

    let mut children = vec![];
    let mut current = at + 2;
    for _ in 0 .. nchildren {
        let (child, next) = tree(&source, current);
        children.push(child);
        current = next;
    }

    let end = current + nmetas as usize;
    let meta = source[current..end].to_vec();

    (Tree { children: children, meta: meta }, end)
}

fn p1(root: &Tree) -> i32 {
    let metasum: i32 = root.meta.iter().sum();
    let childrensum: i32 = root.children.iter().map(|v| p1(v)).sum();
    metasum + childrensum
}

fn p2(root: &Tree) -> i32 {
    if root.children.is_empty() {
        root.meta.iter().sum::<i32>()
    } else {
        root.meta.iter()
                 .filter(|&&x| x > 0)
                 .map(|x| x - 1)
                 .filter(|&x| x < root.children.len() as i32)
                 .map(|x| p2(&root.children[x as usize]))
                 .sum()
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock()
                     .lines()
                     .next()
                     .unwrap()
                     .unwrap()
                     ;
    let ints = input.split_whitespace()
                    .map(|s| s.parse::<i32>())
                    .map(|s| s.unwrap())
                    .collect::<Vec<i32>>()
                    ;

    let (t, _) = tree(&ints, 0);
    let meta1 = p1(&t);
    let meta2 = p2(&t);

    println!("P1: {}", meta1);
    println!("P2: {}", meta2);
}
