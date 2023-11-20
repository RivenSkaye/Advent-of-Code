#![feature(test)]
extern crate test;

use aoc2015::common;

fn part_one(input: &[u8]) -> isize {
    let mut r = 0;
    for b in input {
        if b'('.eq(b) {
            r += 1
        } else {
            r -= 1
        }
    }
    r
}

fn part_two(input: &[u8]) -> usize {
    let mut pos = 0;
    let mut floor = 0;
    loop {
        let b = unsafe { input.get_unchecked(pos) };
        if b'('.eq(b) {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor == -1 {
            return pos + 1;
        }
        pos += 1;
    }
}

pub fn main() {
    let data = common::read_file::<1>();
    println!("Part 1: {}", part_one(data.as_bytes()));
    println!("Part 1: {}", part_two(data.as_bytes()));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn p1bench(b: &mut test::Bencher) {
        let input = common::read_file::<1>();
        let bs = input.as_bytes();
        b.iter(|| assert_eq!(part_one(test::black_box(bs)), 74));
    }

    #[bench]
    fn p2bench(b: &mut test::Bencher) {
        let input = common::read_file::<1>();
        let bs = input.as_bytes();
        b.iter(|| assert_eq!(part_two(test::black_box(bs)), 1795))
    }
}
