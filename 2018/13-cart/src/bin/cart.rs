use std::io;
use std::io::prelude::*;
use std::collections::HashSet;
use std::cmp::Ordering;

#[derive(Eq, Clone, Copy)]
struct Cart {
    x: usize,
    y: usize,
    dir: char,
    turn: char,
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        if self.x == other.x {
            self.y.cmp(&other.y)
        } else {
            self.x.cmp(&other.x)
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn maketurn(dir: char, turn: char) -> (char, char) {
    match turn {
        '<' => (match dir {
            '<' => 'v',
            'v' => '>',
            '>' => '^',
            '^' => '<',
             _  => panic!("?? turn") }, '|'),
        '|' => (dir, '>'),
        '>' => (match dir {
            '<' => '^',
            '^' => '>',
            '>' => 'v',
            'v' => '<',
             _  => panic!("?? turn") }, '<'),
        x   => panic!("?? from {}", x)
    }
}

fn tick(map: &Vec<Vec<char>>, prev: &Vec<Cart>) -> Vec<Cart> {
    let mut carts = prev.clone();
    let mut pos = HashSet::new();
    let mut to_remove = vec![];

    for mut cart in carts.iter_mut() {

        if pos.contains(&(cart.x, cart.y)) {
            to_remove.push(cart.clone());
            continue;
        }

        match cart.dir {
            '<' => cart.x -= 1,
            '>' => cart.x += 1,
            '^' => cart.y -= 1,
            'v' => cart.y += 1,
             _  => panic!("? dir")
        }

        if pos.contains(&(cart.x, cart.y)) {
            println!("collision at {},{}", cart.x, cart.y);
            to_remove.push(cart.clone());
        } else {
            pos.insert((cart.x, cart.y));
        }

        match map[cart.y][cart.x] {
            '+'  => {
                let (d, t) = maketurn(cart.dir, cart.turn);
                cart.dir = d;
                cart.turn = t;
            },
            '\\' => cart.dir = match cart.dir {
                    '<'  => '^',
                    '^'  => '<',
                    'v'  => '>',
                    '>'  => 'v',
                     x   => panic!("?? {}", x)
            },
            '/'  => cart.dir = match cart.dir {
                    '^' => '>',
                    '>' => '^',
                    'v' => '<',
                    '<' => 'v',
                     x   => panic!("?? {}", x)
            },
            _    => ()
        }
    }

    carts.sort();
    carts.retain(|cart| to_remove.iter().find(|&x| x == cart).is_none());

    if carts.len() == 1 {
        println!("last car: {},{}", carts[0].x, carts[0].y);
    }

    carts
}

fn main() {
    let stdin = io::stdin();
    let map: Vec<Vec<char>> =
        stdin.lock()
             .lines()
             .map(|s| s.unwrap())
             .map(|s| s.chars().collect())
             .collect()
             ;

    let mut initial = vec![];

    for j in 0 .. map.len() {
        for i in 0 .. map[j].len() {
            let x = match map[j][i] {
                '<' => Some(Cart {x: i, y: j, dir: '<', turn: '<'}),
                '>' => Some(Cart {x: i, y: j, dir: '>', turn: '<'}),
                '^' => Some(Cart {x: i, y: j, dir: '^', turn: '<'}),
                'v' => Some(Cart {x: i, y: j, dir: 'v', turn: '<'}),
                _   => None
            };

            if x.is_some() {
                initial.push(x.unwrap());
            }
        }
    }

    itertools::iterate(initial, |carts| tick(&map, &carts))
        .take_while(|x| x.len() > 1)
        .count()
        ;
}
