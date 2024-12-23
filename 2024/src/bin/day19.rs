#![feature(slice_split_once, test)]
extern crate test;

use std::collections::HashMap;

// Lots of allocations today, this should speed up parsing
use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use aoc2024::common;

pub fn parse(data: &[u8]) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let (avail, requests) = data.split_once(|chr| b'\n'.eq(chr)).unwrap();
    let towels = avail
        .split(|c| b','.eq(c)) // split at the comma
        .map(|towel| towel.trim_ascii().to_owned()) // yeet leading and trailing space
        .collect();
    let patterns = requests
        .trim_ascii()
        .split(|c| b'\n'.eq(c))
        .map(|ptrn| ptrn.to_owned())
        .collect();
    (towels, patterns)
}

fn can_stack(pattern: &[u8], towels: &[Vec<u8>]) -> bool {
    if pattern.is_empty() || towels.iter().any(|towel| towel == pattern) {
        return true;
    }
    towels
        .iter()
        .filter_map(|towel| pattern.strip_prefix(towel.as_slice()))
        .any(|rest| can_stack(rest, towels))
}

pub fn part_one(towels: &[Vec<u8>], patterns: &[Vec<u8>]) -> usize {
    patterns
        .iter()
        .filter(|pattern| can_stack(pattern, towels))
        .count()
}

fn all_stacks<'a>(
    pattern: &'a [u8],
    towels: &[Vec<u8>],
    solutions: &mut HashMap<&'a [u8], usize>,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(&res) = solutions.get(pattern) {
        return res;
    }
    let res = towels
        .iter()
        .filter_map(|towel| pattern.strip_prefix(towel.as_slice()))
        .map(|rest| all_stacks(rest, towels, solutions))
        .sum();
    solutions.insert(pattern, res);
    res
}

pub fn part_two(towels: &[Vec<u8>], patterns: &[Vec<u8>]) -> usize {
    patterns
        .iter()
        .map(|pattern| all_stacks(pattern, towels, &mut HashMap::with_capacity(patterns.len())))
        .sum()
}

pub fn main() {
    let data = common::read_file::<19>();
    let (towels, patterns) = parse(&data);
    println!("{}", part_one(&towels, &patterns));
    println!("{}", part_two(&towels, &patterns));
}
