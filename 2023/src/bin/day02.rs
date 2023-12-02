#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use aoc2023::common;

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

/// Game, max red, max blue, max green
#[derive(Debug, PartialEq, Eq)]
pub struct MaxRGB(usize, usize, usize, usize);

impl From<(usize, usize, usize, usize)> for MaxRGB {
    fn from(value: (usize, usize, usize, usize)) -> Self {
        Self(value.0, value.1, value.2, value.3)
    }
}

pub fn parse(input: &[u8]) -> Vec<MaxRGB> {
    input
        .split(|c| b'\n'.eq(c))
        .map(|line| {
            // after parsing the game number, this will be on the first cube number
            let mut next_start = 6;
            // discard "Game ", get number
            let game = line[5..]
                .iter()
                .map_while(|c| {
                    next_start += 1;
                    b':'.ne(c).then_some(c)
                })
                .fold(0, |i, c| (10 * i) + (c - b'0') as usize);
            let mut res = (game, 0, 0, 0);
            for chunk in line[next_start..]
                .split(|c| b';'.eq(c))
                .map(|ch| ch.trim_ascii())
            {
                let mut color_start = 0;
                while color_start < chunk.len() {
                    let cubecount = chunk[color_start..]
                        .iter()
                        .take_while(|c| {
                            next_start += 1;
                            color_start += 1;
                            c.is_ascii_digit()
                        })
                        .fold(0, |i, c| (10 * i) + (c - b'0') as usize);
                    match chunk[color_start] {
                        b'r' => {
                            res.1 = res.1.max(cubecount);
                            color_start += 4;
                        }
                        b'g' => {
                            res.2 = res.2.max(cubecount);
                            color_start += 6;
                        }
                        b'b' => {
                            res.3 = res.3.max(cubecount);
                            color_start += 5
                        }
                        _ => unreachable!(),
                    }
                    if chunk
                        .get(color_start)
                        .is_some_and(|c| c.is_ascii_whitespace())
                    {
                        color_start += 1;
                    }
                }
            }
            res
        })
        .map(MaxRGB::from)
        .collect()
}

pub fn part_one(parsed: &[MaxRGB]) -> usize {
    parsed
        .iter()
        .filter_map(|game| {
            if !(game.1 > MAX_RED || game.2 > MAX_GREEN || game.3 > MAX_BLUE) {
                Some(game.0)
            } else {
                None
            }
        })
        .sum()
}

pub fn part_two(parsed: &[MaxRGB]) -> usize {
    parsed.iter().map(|game| game.1 * game.2 * game.3).sum()
}

pub fn main() {
    let input = common::read_file::<2>();
    let parsed = parse(&input);
    println!("{}", part_one(&parsed));
    println!("{}", part_two(&parsed));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file::<2>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(parse(test::black_box(&input)), parsed))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_file::<2>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part_one(test::black_box(&parsed)), 2416))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_file::<2>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part_two(test::black_box(&parsed)), 63307))
    }
}
