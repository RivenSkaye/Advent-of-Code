#![feature(slice_split_once, byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use std::collections::HashMap;

use aoc2023::common;

pub fn parse(input: &[u8]) -> Vec<(Vec<usize>, Vec<usize>)> {
    input
        .split(|c| b':'.eq(c))
        .skip(1)
        .map(|card| {
            let (winners, scratched) = card.split_once(|c| b'|'.eq(c)).unwrap();
            let w = winners
                .split(|c| b' '.eq(c))
                .filter(|s| !s.is_empty())
                .map(|w| w.iter().fold(0, |a, b| (a * 10) + (b - b'0') as usize))
                .collect();
            let s = scratched
                .split(|c| b' '.eq(c) || b'\n'.eq(c))
                .filter(|s| !s.is_empty())
                .take_while(|s| b'C' != s[0])
                .map(|s| s.iter().fold(0, |a, b| (a * 10) + (b - b'0') as usize))
                .collect();
            (w, s)
        })
        .collect()
}

pub fn part_one(parsed: &[(Vec<usize>, Vec<usize>)]) -> usize {
    parsed
        .iter()
        .map(|(winners, scratched)| {
            scratched
                .iter()
                .filter(|n| winners.contains(n))
                .count()
                .checked_sub(1)
                .map(|n| 2usize.pow(n as u32))
                .unwrap_or_default()
        })
        .sum()
}

pub fn part_two(parsed: &[(Vec<usize>, Vec<usize>)]) -> usize {
    let mut wins = HashMap::<usize, usize>::with_capacity(parsed.len() * 2);
    (1..)
        .zip(parsed.iter())
        .map(|(next, (winners, scratched))| {
            // base amount of wins
            let n = scratched.iter().filter(|n| winners.contains(n)).count();
            // how many copies of this card we got
            let dups = wins.get(&next).unwrap_or(&0) + 1;
            // for the next n cards...
            for k in (next + 1)..=(next + n) {
                // add a copy for every duplicate of this card
                wins.insert(k, wins.get(&k).unwrap_or(&0) + dups);
            }
            dups
        })
        .sum()
}

pub fn main() {
    let input = common::read_file::<4>();
    let parsed = parse(&input);
    println!("{}", part_one(&parsed));
    println!("{}", part_two(&parsed));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file::<4>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(parse(test::black_box(&input)), parsed))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_file::<4>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part_one(test::black_box(&parsed)), 26218))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_file::<4>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part_two(test::black_box(&parsed)), 9997537))
    }
}
