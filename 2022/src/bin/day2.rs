#![feature(test)]
extern crate test;

use aoc2022::common;

pub fn get_data(data: &str) -> Vec<(u8, u8)> {
    data.lines()
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
            (b'A', b'X') => 4,
            (b'A', b'Y') => 8,
            (b'A', b'Z') => 3,
            (b'B', b'X') => 1,
            (b'B', b'Y') => 5,
            (b'B', b'Z') => 9,
            (b'C', b'X') => 7,
            (b'C', b'Y') => 2,
            (b'C', b'Z') => 6,
            (_, _) => unreachable!(),
        })
        .sum()
}

pub fn part_two(input: Vec<(u8, u8)>) -> usize {
    input
        .iter()
        .map(|round| match round {
            (b'A', b'X') => 3,
            (b'A', b'Y') => 4,
            (b'A', b'Z') => 8,
            (b'B', b'X') => 1,
            (b'B', b'Y') => 5,
            (b'B', b'Z') => 9,
            (b'C', b'X') => 2,
            (b'C', b'Y') => 6,
            (b'C', b'Z') => 7,
            (_, _) => unreachable!(),
        })
        .sum()
}

pub fn main() {
    let input = common::read_file(2);
    println!("{}", part_one(get_data(&input)));
    println!("{}", part_two(get_data(&input)));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file(2);
        b.iter(|| get_data(test::black_box(&input)))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let read = common::read_file(2);
        let input = get_data(&read);
        b.iter(|| assert_eq!(part_one(test::black_box(input.clone())), 13565))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let read = common::read_file(2);
        let input = get_data(&read);
        b.iter(|| assert_eq!(part_two(test::black_box(input.clone())), 12424))
    }
}
