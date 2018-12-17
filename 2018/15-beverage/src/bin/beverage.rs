use std::io;
use std::io::prelude::*;

extern crate itertools;

#[derive(Clone)]
struct Game {
    cells: Vec<char>,
    rows: usize,
    cols: usize,
    hp: HashMap<usize, i32>,
}

type Coord = usize;

fn neighbours(board: &Game, pos: Coord) -> Vec<Coord> {
    let mut v = vec![];

    let rows = board.rows;
    let cols = board.cols;

    let i = pos % cols;
    let j = pos / cols;

    if i != 0        { v.push(pos - 1) }
    if i != cols - 1 { v.push(pos + 1) }
    if j != 0        { v.push(pos - cols) }
    if j != rows - 1 { v.push(pos + cols) }

    v
}

fn filterplayers(board: &Game) -> Vec<(Coord, char)> {
    board.cells
          .iter()
          .enumerate()
          .filter(|(_, &c)| c == 'E' || c == 'G')
          .map(|(i, &c)| (i, c))
          .collect()
}

fn filterelves(xs: &Vec<(Coord, char)>) -> Vec<(Coord, char)> {
    xs.iter().filter(|(_, c)| *c == 'E').map(|x| x.clone()).collect()
}

fn filtergoblins(xs: &Vec<(Coord, char)>) -> Vec<(Coord, char)> {
    xs.iter().filter(|(_, c)| *c == 'G').map(|x| x.clone()).collect()
}

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::usize;
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: usize,
    parent: usize,
}

/* 
 * turn comparison, because binary_heap is a max-heap, but dijkstra needs a
 * min-heap shortest-path reading-order is always preferred because of index
 * row-by-col representation of positions
 */
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        self.cost.cmp(&other.cost)
            .then_with(|| self.pos.cmp(&other.pos))
            .reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(board: &Game, src: Coord, dst: Coord) -> Option<State> {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut heritage = HashMap::new();

    dist.insert(src, 0);
    heap.push(State { cost: 0, pos: src, parent: src });
    heritage.insert(src, src);

    let goals = neighbours(&board, dst);

    /* 
     * if already next to an enemy, there's no better shortest path,
     * so return a no-op move
     */
    if goals.iter().any(|&x| x == src) {
        return Some(State { cost: 0, pos: src, parent: src })
    }

    while let Some(State { cost, pos, parent: _ }) = heap.pop() {

        if goals.iter().any(|&x| x == pos) {

            /* backtrack to parent to find out which step to make first */
            let mut parent = pos;
            while let Some(p) = heritage.get(&parent) {
                if *p == src { break }
                parent = *p;
            }

            return Some(State { cost: cost, pos: pos, parent: parent });
        }

        if cost > *dist.get(&pos).unwrap_or(&usize::MAX) {
            continue
        }

        for edge in neighbours(&board, pos) {
            /* impassable (wall or other player), so don't consider */
            if board.cells[edge] != '.' { continue }

            let next = State { cost: cost + 1, pos: edge, parent: pos };

            if next.cost < *dist.get(&edge).unwrap_or(&usize::MAX) {
                heap.push(next);
                dist.insert(edge, next.cost);
                heritage.insert(edge, pos);
            }
        }
    }

    None
}

fn walk(prev: &Game, player: (Coord, char)) -> (Game, Coord) {

    let mut game = prev.clone();
    let players = filterplayers(&game);
    let enemies = match player.1 {
        'G' => filterelves(&players),
        'E' => filtergoblins(&players),
         x  => panic!("found player {}", x),
    };

    let coord = player.0;
    let mut nearest = enemies.iter()
                                .map(|x| shortest_path(&game, coord, x.0))
                                .flat_map(|x| x)
                                .collect::<Vec<_>>()
                                ;

    /* none reachable, so skip */
    if nearest.is_empty() { return (game, coord) }

    /* 
        * prefer going up, then left, because it would place higher on
        * reading order
        */
    nearest.sort_by(|&lhs, &rhs| lhs.cost.cmp(&rhs.cost));

    let cost = nearest.first().unwrap().cost;
    if cost == 0 { return (game, coord) }
    let mut targets = nearest.iter()
                              .take_while(|x| x.cost == cost)
                              .collect::<Vec<_>>()
                              ;
    targets.sort_by(|&lhs, &rhs| lhs.parent.cmp(&rhs.parent));

    let target = targets.first().unwrap();
    if target.parent == coord { return (game, coord) }

    let hp = game.hp.remove(&coord).unwrap();
    game.hp.insert(target.parent, hp);
    game.cells[target.parent] = player.1;
    game.cells[coord] = '.';

    (game, target.parent)
}

use itertools::Itertools;

fn fight(prev: &Game, player: (Coord, char), attack_power: i32) -> Game {
    let mut game = prev.clone();

    // unit may have been killed this round
    if game.cells[player.0] == '.' { return game }

    let kind = player.1;
    let target = neighbours(&game, player.0)
        .iter()
        .map(|x| *x)
        .filter(|&x|(game.cells[x] == 'G' && kind == 'E')
                 || (game.cells[x] == 'E' && kind == 'G'))
        .map(|x| (game.hp[&x], x))
        .sorted()
        .next()
        ;

    if target.is_none() { return game }

    let (hp, pos)  = target.unwrap();
    if hp - attack_power >= 0 {
        game.hp.insert(pos, hp - attack_power);
    } else {
        game.hp.remove(&pos);
        game.cells[pos] = '.';
    }

    game
}

fn print(board: &Game) -> () {
    for chunk in board.cells.chunks(board.cols) {
        println!("{}", chunk.iter().collect::<String>());
    }
    println!("");
}

fn game1(mut game: Game) -> (i32, i32) {
    let mut rounds = 0;

    'game: loop {
        print(&game);

        for player in filterplayers(&game) {
            let (next, coord) = walk(&game, player);
            game = fight(&next, (coord, player.1), 3);

            let current = filterplayers(&game);
            let elves   = filterelves(&current);
            let goblins = filtergoblins(&current);

            if player.1 == 'E' && goblins.is_empty() { break 'game }
            if player.1 == 'G' && elves.is_empty()   { break 'game }
        }

        rounds += 1;
    }

    print(&game);

    (game.hp.values().sum::<i32>() * rounds, rounds)
}

fn game2(init_game: Game) -> (i32, i32) {
    let init_elves = init_game.cells.iter().filter(|&&c| c == 'E').count();

    let mut game;
    let mut elf_atk = 3;
    let mut rounds;

    'game: loop {
        elf_atk += 1;
        game = init_game.clone();
        rounds = 0;

        println!("Trying elf attack power {}", elf_atk);
        'battle: loop {

            for player in filterplayers(&game) {
                let (next, coord) = walk(&game, player);
                let atk = if player.1 == 'E' { elf_atk } else { 3 };

                game = fight(&next, (coord, player.1), atk);

                let current = filterplayers(&game);
                let elves   = filterelves(&current);
                let goblins = filtergoblins(&current);

                /* an elf died - increase power, go next round */
                if elves.len() != init_elves { break 'battle }

                if player.1 == 'E' && goblins.is_empty() { break 'game }
            }

            rounds += 1;
        }
    }

    (game.hp.values().sum::<i32>() * rounds, rounds)
}

fn main() {
    let stdin = io::stdin();
    let raw = stdin.lock()
                   .lines()
                   .map(|x| x.unwrap())
                   .map(|x| x.chars().collect::<Vec<char>>())
                   .collect::<Vec<_>>()
                   ;

    let cells = raw.iter().flat_map(|x| x.clone()).collect::<Vec<_>>();
    let initial_hps = cells.iter()
                           .enumerate()
                           .filter(|(_, &c)| c == 'G' || c == 'E')
                           .map(|(i, _)| (i, 200))
                           .collect()
                           ;

    let game = Game {
        cells: cells,
        rows: raw.len(),
        cols: raw[0].len(),
        hp: initial_hps,
    };

    let (outcome1, rounds1) = game1(game.clone());
    let (outcome2, rounds2) = game2(game.clone());

    println!("Outcome (1): {} after {} rounds", outcome1, rounds1);
    println!("Outcome (2): {} after {} rounds", outcome2, rounds2);
}
