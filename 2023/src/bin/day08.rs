#![feature(test)]
extern crate test;

use std::collections::HashMap;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use aoc2023::common;

const P1_START: u32 = 4276545; // b"AAA".iter().fold(0, |a, &b| (a<<8) | b as u32);
const P1_END: u32 = 5921370; // b"ZZZ".iter().fold(0, |a, &b| (a<<8) | b as u32);

const P2_START: u32 = b'A' as u32;
const P2_END: u32 = b'Z' as u32;

pub fn parse(input: &[u8]) -> (Vec<usize>, HashMap<u32, [u32; 2]>) {
    let mut lines = input.split(|c| b'\n'.eq(c));
    // LRLRLRLRLRLRLRLRLR
    let directions = lines
        .next()
        .unwrap()
        .iter()
        .map(|direction| if b'L'.eq(direction) { 0 } else { 1 })
        .collect();
    // XXX = (YYY, ZZZ)
    let maps = lines
        .skip(1) // skip the empty line
        .map(|line| {
            (
                line[..3]
                    .iter()
                    .fold(0, |prev, &cur| (prev << 8) | cur as u32),
                [
                    line[7..10]
                        .iter()
                        .fold(0, |prev, &cur| (prev << 8) | cur as u32),
                    line[12..15]
                        .iter()
                        .fold(0, |prev, &cur| (prev << 8) | cur as u32),
                ],
            )
        })
        .collect();
    (directions, maps)
}

#[inline(always)]
fn solve<const END: u32>(
    mut location: u32,
    directions: &[usize],
    maps: &HashMap<u32, [u32; 2]>,
) -> usize {
    let dirmax = directions.len();
    (1..)
        .zip(directions.iter().cycle())
        .find(
            |(_, &dir)| match maps.get(&location).unwrap()[dir % dirmax] {
                l if (l & END) == END => true,
                l => {
                    location = l;
                    false
                }
            },
        )
        .unwrap()
        .0
}

pub fn part_one(directions: &[usize], maps: &HashMap<u32, [u32; 2]>) -> usize {
    solve::<P1_END>(P1_START, directions, maps)
}

fn gcd(lhs: usize, rhs: usize) -> usize {
    let mut a = lhs;
    let mut b = rhs;
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}

#[inline(always)]
fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

pub fn part_two(directions: &[usize], maps: &HashMap<u32, [u32; 2]>) -> usize {
    maps.keys()
        .filter_map(|&k| ((k & 255) == P2_START).then_some(k))
        .map(|loc| solve::<P2_END>(loc.to_owned(), directions, maps))
        .fold(1, |last, cur| lcm(last, cur))
}

pub fn main() {
    let data = common::read_file::<8>();
    let (dirs, maps) = parse(&data);
    println!("{}", part_one(&dirs, &maps));
    println!("{}", part_two(&dirs, &maps));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;
    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file::<8>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(parse(test::black_box(&input)), parsed))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_file::<8>();
        let (dirs, maps) = parse(&input);
        b.iter(|| {
            assert_eq!(
                part_one(test::black_box(&dirs), test::black_box(&maps)),
                11567
            )
        })
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_file::<8>();
        let (dirs, maps) = parse(&input);
        b.iter(|| {
            assert_eq!(
                part_two(test::black_box(&dirs), test::black_box(&maps)),
                9858474970153
            )
        })
    }
}
