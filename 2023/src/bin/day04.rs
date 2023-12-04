#![feature(slice_split_once, byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

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
                .take_while(|s| !s.starts_with(b"Card"))
                .map(|s| s.iter().fold(0, |a, b| (a * 10) + (b - b'0') as usize))
                .collect();
            (w, s)
        })
        .collect()
}

pub fn part_one(parsed: &[(Vec<usize>, Vec<usize>)]) -> usize {
    parsed
        .iter()
        .filter_map(|(winners, scratched)| {
            match scratched.iter().filter(|n| winners.contains(n)).count() {
                0 => None,
                n => Some(2usize.pow((n - 1) as u32)),
            }
        })
        .sum()
}

pub fn main() {
    let input = common::read_file::<4>();
    let parsed = parse(&input);
    println!("{}", part_one(&parsed));
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
    /*
    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_file::<4>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part_two(test::black_box(&parsed)), 63307))
    }*/
}
