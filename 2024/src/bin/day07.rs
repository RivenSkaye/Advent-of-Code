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
            let parts = line.split_once(|c| b':'.eq(c)).unwrap();
            (
                common::stoi(parts.0),
                parts
                    .1
                    .split(|c| c.is_ascii_whitespace())
                    .filter(|x| x.len() > 0)
                    .map(common::stoi)
                    .collect(),
            )
        })
        .collect()
}

#[inline(always)]
fn try_calcs(res: usize, bits: &[usize]) -> bool {
    (0..(1 << bits.len() - 1)).any(|op| {
        bits[1..]
            .iter()
            .enumerate()
            .fold(bits[0], |curr, (idx, &next)| match (op >> idx) & 1 {
                1 => curr * next,
                _ => curr + next,
            })
            == res
    })
}

pub fn part_one(parsed: &[(usize, Vec<usize>)]) -> usize {
    parsed
        .iter()
        .filter_map(|(res, bits)| try_calcs(*res, bits).then_some(res))
        .sum()
}

pub fn part_two(parsed: &[(usize, Vec<usize>)]) -> usize {
    parsed
        .iter()
        .filter_map(|(res, bits)| {
            (0..(3usize.pow(bits.len() as u32) - 1))
                .any(|op| {
                    bits[1..]
                        .iter()
                        .enumerate()
                        .fold(bits[0], |curr, (idx, &next)| match ((op >> idx) & 3) % 3 {
                            1 => curr * next,
                            0 => curr + next,
                            _ => curr * 10usize.pow(next.ilog10() + 1) + next,
                        })
                        .eq(res)
                })
                .then_some(res)
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

    /*
    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_file::<6>();
        let pos = parse(&input);
        let mut parsed = Grid::from(input);
        b.iter(|| {
            assert_eq!(
                part_two(test::black_box(&mut parsed), test::black_box(pos.clone())),
                1723
            )
        })
    } */
}
