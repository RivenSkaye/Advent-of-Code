#![feature(slice_split_once, test)]
extern crate test;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use aoc2024::common;

pub fn parse(input: &[u8]) -> Vec<(usize, Vec<usize>)> {
    input
        .split(|c| b'\n'.eq(c))
        .map(|line| {
            line.split_once(|c| b':'.eq(c))
                .map(|(a, b)| {
                    (
                        common::stoi(a),
                        b.split(|c| c.is_ascii_whitespace())
                            .filter(|x| x.len() > 0)
                            .map(common::stoi)
                            .collect(),
                    )
                })
                .unwrap()
        })
        .collect()
}

const OPS1: [fn(usize, usize) -> usize; 2] = [|a, b| a + b, |a, b| a * b];
const OPS2: [fn(usize, usize) -> usize; 3] = [
    |a, b| a * b,
    |a, b| a + b,
    |a, b| a * 10usize.pow(b.ilog10() + 1) + b,
];

#[inline(always)]
fn try_calcs_p1(res: usize, curr: usize, bits: &[usize]) -> bool {
    match bits.len() {
        0 => curr == res,
        _ => {
            if curr > res {
                return false;
            }
            OPS1.iter()
                .any(|op| try_calcs_p1(res, op(curr, bits[0]), &bits[1..]))
        }
    }
}

pub fn part_one(parsed: &[(usize, Vec<usize>)]) -> usize {
    parsed
        .iter()
        .filter_map(|(res, bits)| try_calcs_p1(*res, bits[0], &bits[1..]).then_some(res))
        .sum()
}

#[inline(always)]
fn try_calcs_p2(res: usize, curr: usize, bits: &[usize]) -> bool {
    match bits.len() {
        0 => curr == res,
        _ => {
            if curr > res {
                return false;
            }
            OPS2.iter()
                .any(|op| try_calcs_p2(res, op(curr, bits[0]), &bits[1..]))
        }
    }
}

pub fn part_two(parsed: &[(usize, Vec<usize>)]) -> usize {
    parsed
        .iter()
        .filter_map(|(res, bits)| {
            try_calcs_p1(*res, bits[0], &bits[1..])
                .then_some(res)
                .or_else(|| try_calcs_p2(*res, bits[0], &bits[1..]).then_some(res))
        })
        .sum()
}

pub fn main() {
    let input = common::read_file::<7>();
    let parsed = parse(&input);
    println!("{}", part_one(&parsed));
    println!("{}", part_two(&parsed));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file::<7>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(parse(test::black_box(&input)), parsed))
    }
    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_file::<7>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part_one(test::black_box(&parsed)), 1545311493300))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_file::<7>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part_two(test::black_box(&parsed)), 169122112716571))
    }
}
