use std::io;
use std::io::prelude::*;

extern crate regex;
extern crate itertools;

type Memory = Vec<usize>;
type Opcode = fn(Memory, usize, usize, usize) -> Memory;

fn addr(mut mem: Memory, a: usize, b: usize, c: usize) -> Memory {
    mem[c] = mem[a] + mem[b];
    mem
}
fn addi(mut mem: Memory, a: usize, b: usize, c: usize) -> Memory {
    mem[c] = mem[a] + b;
    mem
}

fn mulr(mut mem: Memory, a: usize, b: usize, c: usize) -> Memory {
    mem[c] = mem[a] * mem[b];
    mem
}
fn muli(mut mem: Memory, a: usize, b: usize, c: usize) -> Memory {
    mem[c] = mem[a] * b;
    mem
}

fn banr(mut mem: Memory, a: usize, b: usize, c: usize) -> Memory {
    mem[c] = mem[a] & mem[b];
    mem
}
fn bani(mut mem: Memory, a: usize, b: usize, c: usize) -> Memory {
    mem[c] = mem[a] & b;
    mem
}

fn borr(mut mem: Memory, a: usize, b: usize, c: usize) -> Memory {
    mem[c] = mem[a] | mem[b];
    mem
}
fn bori(mut mem: Memory, a: usize, b: usize, c: usize) -> Memory {
    mem[c] = mem[a] | b;
    mem
}

fn setr(mut mem: Memory, a: usize, _: usize, c: usize) -> Memory {
    mem[c] = mem[a];
    mem
}
fn seti(mut mem: Memory, a: usize, _: usize, c: usize) -> Memory {
    mem[c] = a;
    mem
}

fn gtir(mut mem: Memory, a: usize, b: usize, c: usize) -> Memory {
    mem[c] = if a > mem[b] { 1 } else { 0 };
    mem
}
fn gtri(mut mem: Memory, a: usize, b: usize, c: usize) -> Memory {
    mem[c] = if mem[a] > b { 1 } else { 0 };
    mem
}
fn gtrr(mut mem: Memory, a: usize, b: usize, c: usize) -> Memory {
    mem[c] = if mem[a] > mem[b] { 1 } else { 0 };
    mem
}

fn eqir(mut mem: Memory, a: usize, b: usize, c: usize) -> Memory {
    mem[c] = if a == mem[b] { 1 } else { 0 };
    mem
}
fn eqri(mut mem: Memory, a: usize, b: usize, c: usize) -> Memory {
    mem[c] = if mem[a] == b { 1 } else { 0 };
    mem
}
fn eqrr(mut mem: Memory, a: usize, b: usize, c: usize) -> Memory {
    mem[c] = if mem[a] == mem[b] { 1 } else { 0 };
    mem
}

fn p1(samples: Vec<Vec<String>>) -> usize {
    let snapshot_expr = r".*\[(\d), (\d), (\d), (\d)\]";
    let re = regex::Regex::new(snapshot_expr).unwrap();

    let ops = vec![
        addr as Opcode, addi as Opcode,
        mulr as Opcode, muli as Opcode,
        banr as Opcode, bani as Opcode,
        borr as Opcode, bori as Opcode,
        setr as Opcode, seti as Opcode,
        gtir as Opcode, gtri as Opcode, gtrr as Opcode,
        eqir as Opcode, eqri as Opcode, eqrr as Opcode
    ];

    samples.iter().map(|sample| {
        let in_before = &sample[0];
        let in_after  = &sample[2];
        let in_op     = sample[1].split_whitespace()
                                 .map(|x| x.parse().unwrap())
                                 .collect::<Vec<_>>()
                                 ;

        let before = re.captures(&in_before)
                         .map(|x| vec![x[1].parse().unwrap(),
                                       x[2].parse().unwrap(),
                                       x[3].parse().unwrap(),
                                       x[4].parse().unwrap()])
                         .unwrap()
                         ;
        let after  = re.captures(&in_after)
                         .map(|x| vec![x[1].parse().unwrap(),
                                       x[2].parse().unwrap(),
                                       x[3].parse().unwrap(),
                                       x[4].parse().unwrap()])
                         .unwrap()
                         ;

        let opcode = in_op[0];
        let (a, b, c) = (in_op[1], in_op[2], in_op[3]);

        ops.iter()
           .filter(|op| op(before.clone(), a, b, c) == after)
           .count()
    }).filter(|&x| x >= 3).count()
}

use std::ops::IndexMut;
use itertools::Itertools;

fn find_opcodes(samples: Vec<Vec<String>>) -> Vec<Opcode> {
    let snapshot_expr = r".*\[(\d), (\d), (\d), (\d)\]";
    let re = regex::Regex::new(snapshot_expr).unwrap();

    let ops = vec![
        addr as Opcode, addi as Opcode,
        mulr as Opcode, muli as Opcode,
        banr as Opcode, bani as Opcode,
        borr as Opcode, bori as Opcode,
        setr as Opcode, seti as Opcode,
        gtir as Opcode, gtri as Opcode, gtrr as Opcode,
        eqir as Opcode, eqri as Opcode, eqrr as Opcode
    ];

    let mut possible_ops = (0 .. ops.len())
                                .map(|_| ops.clone())
                                .collect::<Vec<Vec<Opcode>>>()
                                ;

    for sample in samples {
        let in_before = &sample[0];
        let in_after  = &sample[2];
        let in_op     = sample[1].split_whitespace()
                                 .map(|x| x.parse().unwrap())
                                 .collect::<Vec<_>>()
                                 ;

        let before = re.captures(&in_before)
                         .map(|x| vec![x[1].parse().unwrap(),
                                       x[2].parse().unwrap(),
                                       x[3].parse().unwrap(),
                                       x[4].parse().unwrap()])
                         .unwrap()
                         ;
        let after  = re.captures(&in_after)
                         .map(|x| vec![x[1].parse().unwrap(),
                                       x[2].parse().unwrap(),
                                       x[3].parse().unwrap(),
                                       x[4].parse().unwrap()])
                         .unwrap()
                         ;

        let opcode = in_op[0];
        let (a, b, c) = (in_op[1], in_op[2], in_op[3]);
        let candidates: &mut Vec<Opcode> = possible_ops.index_mut(opcode);
        candidates.retain(|op| op(before.clone(), a, b, c) == after);
    }

    while !itertools::all(possible_ops.iter(), |x| x.len() == 1) {
        let unique = possible_ops.iter()
                                 .filter(|ops| ops.len() == 1)
                                 .map(|x| x[0])
                                 .collect::<Vec<_>>()
                                 ;

        for ops in possible_ops.iter_mut() {
            if ops.len() == 1 { continue }
            ops.retain(|op| !itertools::any(unique.iter(), |x| x == op));
        }
    }

    possible_ops.iter().flat_map(|x| x.clone()).collect()
}

fn p2(mem: Memory, ops: Vec<Opcode>, prog: Vec<String>) -> Memory {
    prog.iter().fold(mem, |mem, ln| {
        let code = ln.split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<_>>();
        let op: usize = code[0];
        let  a: usize = code[1];
        let  b: usize = code[2];
        let  c: usize = code[3];
        ops[op](mem, a, b, c)
    })
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock()
                     .lines()
                     .map(|x| x.unwrap())
                     .collect::<Vec<_>>()
                     ;

    let samples = input.chunks(4)
                       .take_while(|x| x[1].len() > 1)
                       .map(|x| x.to_vec())
                       .collect::<Vec<_>>()
                       ;

    let similar = p1(samples.clone());
    println!("{}", similar);

    let opcodes = find_opcodes(samples);
    let program = input.chunks(4)
                       .skip_while(|x| x[1].len() > 1)
                       .flatten()
                       .skip_while(|x| x.is_empty())
                       .map(|x| x.to_string())
                       .collect::<Vec<_>>()
                       ;

    println!("{:?}", p2(vec![0; 4], opcodes, program));
}
