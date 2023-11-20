#![feature(test)]
extern crate test;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use aoc2015::common;

type Measurements = (usize, usize, usize);

fn parse(input: &[u8]) -> Vec<Measurements> {
    input
        .split(|b| b'\n'.eq(b))
        .map(|b| {
            b.split(|x| b'x'.eq(x))
                .map(|inner| {
                    inner
                        .iter()
                        .fold(0, |a, b| (10 * a) + ((b - b'0') as usize))
                })
                .take(3)
                .collect::<Vec<_>>()
        })
        .map(|x| unsafe {
            (
                *x.get_unchecked(0),
                *x.get_unchecked(1),
                *x.get_unchecked(2),
            )
        })
        .collect()
}

fn part_one(input: &Vec<Measurements>) -> usize {
    input.iter().fold(0, |a, b| {
        let x = b.0 * b.1;
        let y = b.1 * b.2;
        let z = b.2 * b.0;
        (2 * (x + y + z)) + x.min(y.min(z)) + a
    })
}

fn part_two(input: &Vec<Measurements>) -> usize {
    input.iter().fold(0, |a, b| {
        (2 * ((b.0 + b.1 + b.2) - b.0.max(b.1.max(b.2)))) + (b.0 * b.1 * b.2) + a
    })
}

pub fn main() {
    let input = common::read_file::<2>();
    let parsed = parse(input.as_bytes());
    println!("{}", part_one(&parsed));
    println!("{}", part_two(&parsed));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file::<2>();
        b.iter(|| parse(test::black_box(input.as_bytes())));
    }

    #[bench]
    fn p1bench(b: &mut test::Bencher) {
        let input = common::read_file::<2>();
        let bs = parse(input.as_bytes());
        b.iter(|| assert_eq!(part_one(test::black_box(&bs)), 1606483));
    }

    #[bench]
    fn p2bench(b: &mut test::Bencher) {
        let input = common::read_file::<2>();
        let bs = parse(input.as_bytes());
        b.iter(|| assert_eq!(part_two(test::black_box(&bs)), 3842356));
    }
}
