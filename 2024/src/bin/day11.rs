#![feature(test)]
extern crate test;

use std::collections::HashMap;

use aoc2024::common::{self, elegant_pair};

pub fn parse(input: &[u8]) -> Vec<usize> {
    input.split(|c| b' '.eq(c)).map(common::stoi).collect()
}

/// Base 10 equivalent of Windows' {HI,LO}{BYTE,WORD,DWORD} macros
/// bitshifting would be equal to using ilog2 here
fn split(stone: usize, mask: usize) -> (usize, usize) {
    // mask = 10usize.pow((stone.ilog10() + 1) / 2)
    let top = stone / mask;
    (top, stone - (top * mask))
}

// p1: blinks = 5
// p2: blinks = 7
fn blink(stone: usize, iter: usize, known: &mut HashMap<u128, usize>) -> usize {
    if iter > 0 {
        let key = elegant_pair(stone << 7, iter);
        if let Some(amount) = known.get(&key) {
            *amount
        } else {
            let count = match stone.checked_ilog10() {
                // Learned this the hard way - it was non-checked ilog first because I'm an idiot
                None => blink(1, iter - 1, known),
                Some(mask) => {
                    // smh can't even cast to a bool
                    if (mask & 1) == 1 {
                        let (left, right) = split(stone, 10usize.pow((mask + 1) / 2));
                        blink(left, iter - 1, known) + blink(right, iter - 1, known)
                    } else {
                        blink(stone * 2024, iter - 1, known)
                    }
                }
            };
            known.insert(key, count);
            count
        }
    } else {
        1
    }
}

pub fn part_one(stones: &[usize]) -> usize {
    let mut known = HashMap::with_capacity(stones.iter().sum());
    stones
        .iter()
        .map(|&stone| blink(stone, 25, &mut known))
        .sum()
}

pub fn part_two(stones: &[usize]) -> usize {
    let mut known = HashMap::with_capacity(stones.iter().sum());
    stones
        .iter()
        .map(|&stone| blink(stone, 75, &mut known))
        .sum()
}

pub fn main() {
    let input = common::read_file::<11>();
    let parsed = parse(&input);
    println!("{}", part_one(&parsed));
    println!("{}", part_two(&parsed));
}
