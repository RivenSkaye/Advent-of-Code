#![feature(iter_intersperse, test)]
extern crate test;

use std::{collections::VecDeque, ops::AddAssign};

use aoc2024::common;

pub fn parse(input: &[u8]) -> ([usize; 3], Vec<usize>) {
    let mut it = input.split(|chr| b'\n'.eq(chr));
    let regs = [
        it.by_ref()
            .map(|reg_a| common::stoi(&reg_a[12..]))
            .next()
            .unwrap(),
        0,
        0,
    ]; // test and real input initialize B and C to 0
    (
        regs,
        it.skip(3) // reg b, reg c, and the empty line
            .map(|line| {
                line[9..]
                    .split(|chr| b','.eq(chr))
                    .map(common::stoi)
                    .collect()
            })
            .next()
            .unwrap(),
    )
}

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

fn combo(operand: usize, registers: &[usize; 3]) -> usize {
    match operand {
        4 => registers[A],
        5 => registers[B],
        6 => registers[C],
        n => n, // covers 0 through 3, and anything >6 is impossible in a well-formed program
    }
}

pub fn execute(
    op: usize,
    operand: usize,
    registers: &mut [usize; 3],
    out: &mut Vec<usize>,
    instruction: &mut usize,
) {
    match op {
        0 => registers[A] /= 2usize.pow(combo(operand, &registers) as _), // a division
        1 => registers[B] ^= operand,                                     // b xor literal
        2 => registers[B] = combo(operand, &registers) % 8,               // b store
        3 => {
            //jump non-zero
            if registers[A] > 0 {
                return *instruction = operand;
            }
        }
        4 => registers[B] ^= registers[C],             // b xor c
        5 => out.push(combo(operand, &registers) % 8), // out
        6 => registers[B] = registers[A] / 2usize.pow(combo(operand, &registers) as _), // b division
        7 => registers[C] = registers[A] / 2usize.pow(combo(operand, &registers) as _), // c division
        _ => unreachable!(),
    }
    instruction.add_assign(2)
}

pub fn part_one(mut registers: [usize; 3], program: &[usize]) -> Vec<usize> {
    let mut instruction = 0;
    let mut out = Vec::with_capacity(512);
    while instruction < program.len() - 1 {
        execute(
            program[instruction],
            program[instruction + 1],
            &mut registers,
            &mut out,
            &mut instruction,
        );
    }
    out
}

pub fn part_two(program: &[usize]) -> usize {
    // I started noticing a pattern between lengths and bits at 2AM, namely that
    // the bit patterns produce one extra output number for every multiple of 3.
    // 0 - 3: 1 output
    // 4 - 6: 2 outputs
    // 7 - 9: 3 outputs
    // So I started fiddling with the bits as well, based on length positions
    // and through trial and error, I found that this number minus one will
    // produce 5 outputs on the test, and 15 for real input.
    // From this point onward, we produce the exact amount of output values in
    // the program as our input.
    // It's still slow, and I have no idea how to optimize it, but it works!
    // let start = ((1 << ((program.len() - 1) * 3)) >> 1) * 2;
    // Never mind, the whole approach was wrong. It reaches a point where
    // certain bit patterns will cause it to grow longer and then shorter again.
    // I could've known this, had I checked a more substantial sample size for
    // the test inputs value space. It apparently takes a couple hundred thousand
    // with a lot of collisions on the same sequence!
    // Instead, since all we care about is every segment of 3 bits, we check the
    // pattern producing a specific output for every position and "store" that
    // by keeping that position 3 bits higher.
    // Fun fact! The output is generated based on the reverse 3-bit segments in
    // the input. So if e.g. the last output is 0, and 5 produces only 0, then
    // the binary representation must START with 0b101.

    let mut tries = VecDeque::with_capacity(6400);
    tries.push_back((0, program.len() - 1));
    while let Some((next, len)) = tries.pop_front() {
        for tail in (0..8).map(|a| (next << 3) | a) {
            // let cmp = part_one([tail, 0, 0], program);
            //println!("{tail} => {:?}", part_one([tail, 0, 0], program));
            // println!("Chasing {tail} => {cmp:?}");
            // Check partial program output
            if part_one([tail, 0, 0], program) == program[len..] {
                // unless this partial happens to be the whole program
                if len == 0 {
                    return tail;
                }
                // We found a match, push the next position in
                tries.push_back((tail, len - 1));
            }
        }
    }
    unreachable!()
}

pub fn main() {
    let data = common::read_file::<17>();
    // No need to make it mut or clone, [usize; 3] is Copy and that'll prevent moves.
    // Printing it before and after p1 will show it's unchanged. Thanks rustc <3
    let (registers, program) = parse(&data);
    println!(
        "{}",
        part_one(registers, &program)
            .iter()
            .map(|&n| (n as u8 + b'0') as char)
            .intersperse(',')
            .collect::<String>()
    );
    println!("{}", part_two(&program));
}
