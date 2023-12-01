#![feature(test)]
extern crate test;

use aoc2023::common;

pub fn part_one(input: &[u8]) -> usize {
    input
        .split(|chr| b'\n'.eq(chr))
        .map(|line| {
            line.iter()
                .filter_map(|chr| chr.is_ascii_digit().then_some((chr - b'0') as usize))
                .collect::<Vec<usize>>()
        })
        .map(|nums| (nums[0] * 10) + nums[nums.len() - 1])
        .sum()
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_two(input: &[u8]) -> usize {
    input
        .split(|chr| b'\n'.eq(chr))
        .map(|line| {
            (line
                .first()
                .is_some_and(|chr| chr.is_ascii_digit())
                .then(|| (line[0] - b'0') as usize)
                .or_else(|| {
                    NUMBERS.iter().zip(1..).find_map(|(number, val)| {
                        line.starts_with(number.as_bytes()).then_some(val)
                    })
                })
                .unwrap()
                * 10)
                + line
                    .last()
                    .is_some_and(|chr| chr.is_ascii_digit())
                    .then(|| (line[0] - b'0') as usize)
                    .or_else(|| {
                        NUMBERS.iter().zip(1..).find_map(|(number, val)| {
                            line.ends_with(number.as_bytes()).then_some(val)
                        })
                    })
                    .unwrap()
        })
        .sum()
}

pub fn main() {
    let data = common::read_file::<1>();
    println!("{}", part_one(&data));
    println!("{}", part_two(&data))
}
