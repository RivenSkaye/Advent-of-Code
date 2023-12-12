#![feature(array_windows)]
#![feature(test)]
extern crate test;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use aoc2023::common;

pub fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| common::stosi(s.as_bytes()))
                .collect()
        })
        .collect()
}

fn solve1(line: &[i64]) -> i64 {
    let nextline = line
        .array_windows::<2>()
        .map(|&[lhs, rhs]| rhs - lhs)
        .collect::<Vec<_>>();
    if nextline.len() < 2 {
        // 1 in => len == 0 => 0
        // 2 in => len == 1 => that one
        *nextline.last().unwrap_or(&0)
        // 3 or more => check if any diffs are not 0
    } else if nextline.iter().any(|num| 0.ne(num)) {
        // non-zero diff => solve again and add to the last in this line
        solve1(&nextline) + line.last().unwrap()
    } else {
        // return the last in this line
        *line.last().unwrap()
    }
}

pub fn part_one(input: &[Vec<i64>]) -> i64 {
    input.iter().map(|line| solve1(line)).sum()
}

pub fn part_two(input: &[Vec<i64>]) -> i64 {
    let mut copy = input.to_owned();
    copy.iter_mut()
        .map(|line| {
            line.reverse();
            solve1(line)
        })
        .sum()
}

pub fn main() {
    let data = common::read_str::<9>();
    let parsed = parse(&data);
    println!("{}", part_one(&parsed));
    println!("{}", part_two(&parsed));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_str::<9>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(parse(test::black_box(&input)), parsed))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_str::<9>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part_one(test::black_box(&parsed)), 1974232246))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_str::<9>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part_two(test::black_box(&parsed)), 928))
    }
}
