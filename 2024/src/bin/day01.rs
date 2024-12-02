#![feature(iter_array_chunks, test)]
extern crate test;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use aoc2024::common;

pub fn parse(input: &[u8]) -> (Vec<usize>, Vec<usize>) {
    let mut left = Vec::<_>::with_capacity(input.len());
    let mut right = Vec::<_>::with_capacity(input.len());
    for [l, r] in input
        .split(|chr| chr.is_ascii_whitespace())
        .filter(|word| word.len() > 0)
        .array_chunks::<2>()
    {
        left.push(common::stoi(l));
        right.push(common::stoi(r));
    }
    left.sort();
    right.sort();
    (left, right)
}

pub fn part_one(left: &Vec<usize>, right: &Vec<usize>) -> usize {
    left.into_iter()
        .zip(right.into_iter())
        .map(|(ln, &rn)| ln.abs_diff(rn) as usize)
        .sum()
}

pub fn part_two(left: &Vec<usize>, right: &Vec<usize>) -> usize {
    let mut last = usize::MAX;
    let mut mem = 0;
    left.into_iter().fold(0, |prev, num| {
        if last.ne(num) {
            mem = right.into_iter().filter(|n| num.eq(n)).count() * num;
            last = *num;
        }
        mem + prev
    })
}

pub fn main() {
    let data = common::read_file::<1>();
    let (lvec, rvec) = parse(&data);
    println!("{}", part_one(&lvec, &rvec));
    println!("{}", part_two(&lvec, &rvec));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file::<1>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(parse(test::black_box(&input)), parsed))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_file::<1>();
        let parsed = parse(&input);
        b.iter(|| {
            assert_eq!(
                part_one(test::black_box(&parsed.0), test::black_box(&parsed.1)),
                2031679
            )
        })
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_file::<1>();
        let parsed = parse(&input);
        b.iter(|| {
            assert_eq!(
                part_two(test::black_box(&parsed.0), test::black_box(&parsed.1)),
                19678534
            )
        })
    }
}
