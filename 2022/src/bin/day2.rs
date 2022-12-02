#![feature(test)]
extern crate test;

use aoc2022::common;

pub fn get_data() -> Vec<(u8, u8)> {
    common::read_file(2)
        .lines()
        .map(|line| {
            let moves = line.as_bytes();
            (moves[0], moves[2])
        })
        .collect()
}

pub fn part_one(input: Vec<(u8, u8)>) -> usize {
    input
        .iter()
        .map(|round| match round {
            (b'A', b'X') => 3 + 1,
            (b'A', b'Y') => 6 + 2,
            (b'A', b'Z') => 0 + 3,
            (b'B', b'X') => 0 + 1,
            (b'B', b'Y') => 3 + 2,
            (b'B', b'Z') => 6 + 3,
            (b'C', b'X') => 6 + 1,
            (b'C', b'Y') => 0 + 2,
            (b'C', b'Z') => 3 + 3,
            (_, _) => unreachable!(),
        })
        .sum()
}

pub fn part_two(input: Vec<(u8, u8)>) -> usize {
    input
        .iter()
        .map(|round| match round {
            (b'A', b'X') => 3 + 0,
            (b'A', b'Y') => 1 + 3,
            (b'A', b'Z') => 2 + 6,
            (b'B', b'X') => 1 + 0,
            (b'B', b'Y') => 2 + 3,
            (b'B', b'Z') => 3 + 6,
            (b'C', b'X') => 2 + 0,
            (b'C', b'Y') => 3 + 3,
            (b'C', b'Z') => 1 + 6,
            (_, _) => unreachable!(),
        })
        .sum()
}

pub fn main() {
    println!("{}", part_one(get_data()));
    println!("{}", part_two(get_data()));
}
