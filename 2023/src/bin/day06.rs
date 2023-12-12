#![feature(test)]
extern crate test;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use aoc2023::common;

pub fn parse(input: &str) -> (Vec<(f64, f64)>, (f64, f64)) {
    input
        .split_once('\n')
        .map(|(time, dist)| {
            (
                time.split_once(':').unwrap().1,
                dist.split_once(':').unwrap().1.trim_end(),
            )
        })
        .map(|(time, dist)| {
            (
                time.split_ascii_whitespace()
                    .zip(dist.split_ascii_whitespace())
                    .map(|(t, d)| (common::stoi(t) as f64, (common::stoi(d) + 1) as f64))
                    .collect(),
                (
                    (time
                        .bytes()
                        .filter(|c| c.is_ascii_digit())
                        .fold(0, |a, b| (10 * a) + (b - b'0') as usize)) as f64,
                    ((dist
                        .bytes()
                        .filter(|c| c.is_ascii_digit())
                        .fold(0, |a, b| (10 * a) + (b - b'0') as usize))
                        + 1) as f64,
                ),
            )
        })
        .unwrap()
}

#[inline(always)]
fn solve((time, dist): &(f64, f64)) -> usize {
    (((time / 2.0) + ((time * time / 4.0) - dist).sqrt()).floor()
        - ((time / 2.0) - ((time * time / 4.0) - dist).sqrt()).ceil()) as usize
        + 1
}

pub fn part_one(input: &[(f64, f64)]) -> usize {
    input.iter().map(solve).product()
}

pub fn part_two(pair: &(f64, f64)) -> usize {
    solve(pair)
}

pub fn main() {
    let data = common::read_str::<6>();
    let parsed = parse(&data);
    println!("{}", part_one(&parsed.0));
    println!("{parsed:?}");
    println!("{}", part_two(&parsed.1));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_str::<6>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(parse(test::black_box(&input)), parsed))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_str::<6>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part_one(test::black_box(&parsed.0)), 220320))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_str::<6>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part_two(test::black_box(&parsed.1)), 34454850))
    }
}
