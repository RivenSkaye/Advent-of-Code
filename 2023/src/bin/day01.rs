#![feature(test)]
extern crate test;

use aoc2023::common;
use std::iter::Iterator;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub fn parse(input: &[u8]) -> Vec<&[u8]> {
    input.split(|chr| b'\n'.eq(chr)).collect()
}

pub fn part_one<'a>(input: impl Iterator<Item = &'a &'a [u8]>) -> usize {
    input
        .map(|line| {
            line.iter()
                .find_map(|chr| chr.is_ascii_digit().then_some((chr - b'0') as usize * 10))
                .unwrap()
                + line
                    .iter()
                    .rev()
                    .find_map(|chr| chr.is_ascii_digit().then_some((chr - b'0') as usize))
                    .unwrap()
        })
        .sum()
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

// hint for inlining, use always to force
#[inline]
fn get_start(line: &[u8], idx: usize) -> Option<usize> {
    NUMBERS.iter().zip(1..).find_map(|(num, val)| {
        line[idx..]
            .starts_with(num.as_bytes())
            .then_some(val)
            .or_else(|| {
                line[idx]
                    .is_ascii_digit()
                    .then(|| (line[idx] - b'0') as usize)
            })
            .map(|v| v * 10)
    })
}

#[inline]
fn get_end(line: &[u8], idx: usize) -> Option<usize> {
    NUMBERS.iter().zip(1..).find_map(|(num, val)| {
        line[..=idx]
            .ends_with(num.as_bytes())
            .then_some(val)
            .or_else(|| {
                line[idx]
                    .is_ascii_digit()
                    .then(|| (line[idx] - b'0') as usize)
            })
    })
}

pub fn part_two<'a>(input: impl Iterator<Item = &'a &'a [u8]>) -> usize {
    input
        .map(|line| {
            (0..line.len())
                .find_map(|idx| get_start(line, idx))
                .unwrap()
                + (0..line.len())
                    .rev()
                    .find_map(|idx| get_end(line, idx))
                    .unwrap()
        })
        .sum()
}

pub fn main() {
    let data = common::read_file::<1>();
    let parsed = parse(&data);
    // println!("{}", part_one(parsed.iter()));
    // Due to the example datasets being different, part one PANICS.
    // Replace test_inputs/day01.txt with the following:
    // 1abc2
    // pqr3stu8vwx
    // a1b2c3d4e5f
    // treb7uchet
    println!("{}", part_two(parsed.iter()))
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file::<1>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(parse(test::black_box(&input)), parsed))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_file::<1>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part_one(test::black_box(parsed.iter())), 54968))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_file::<1>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part_two(test::black_box(parsed.iter())), 54094))
    }
}
