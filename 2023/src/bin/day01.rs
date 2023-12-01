#![feature(test)]
extern crate test;

use aoc2023::common;

pub fn part_one(input: &[u8]) -> usize {
    input
        .split(|chr| b'\n'.eq(chr))
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

pub fn part_two(input: &[u8]) -> usize {
    input
        .split(|chr| b'\n'.eq(chr))
        .map(|line| {
            (0..line.len())
                .find_map(|idx| {
                    NUMBERS.iter().zip(1..).find_map(|(num, val)| {
                        line[idx..]
                            .starts_with(num.as_bytes())
                            .then_some(val * 10)
                            .or_else(|| {
                                line[idx]
                                    .is_ascii_digit()
                                    .then(|| (line[idx] - b'0') as usize * 10)
                            })
                    })
                })
                .unwrap()
                + (0..line.len())
                    .rev()
                    .find_map(|idx| {
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
                    })
                    .unwrap()
        })
        .sum()
}

pub fn main() {
    let data = common::read_file::<1>();
    // println!("{}", part_one(&data));
    // Due to the example datasets being different, part one PANICS.
    // Replace test_inputs/day01.txt with the following:
    // 1abc2
    // pqr3stu8vwx
    // a1b2c3d4e5f
    // treb7uchet
    println!("{}", part_two(&data))
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_file::<1>();
        b.iter(|| assert_eq!(part_one(test::black_box(&input)), 54968))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_file::<1>();
        b.iter(|| assert_eq!(part_two(test::black_box(&input)), 54094))
    }
}
