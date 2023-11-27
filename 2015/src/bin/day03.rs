#![feature(test)]
extern crate test;

use std::collections::HashSet;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use aoc2015::common;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Point(isize, isize);

impl Point {
    fn step(&mut self, s: &u8) -> &Self {
        match s {
            b'^' => self.1 += 1,
            b'v' => self.1 -= 1,
            b'>' => self.0 += 1,
            b'<' => self.0 -= 1,
            _ => unreachable!()
        };
        self
    }
}

const START: Point = Point(0,0);

fn part_one(input: &[u8]) -> usize {
    let mut last = START;
    let mut points = HashSet::with_capacity(5200);
    points.insert(last);
    input.iter().for_each(|step| {
        points.insert(*last.step(step));
    });
    points.len()
}

fn main() {
    let bs = common::read_file::<3>().as_bytes().to_owned();
    println!("{}", part_one(&bs))
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn p1bench(b: &mut test::Bencher) {
        let bs = common::read_file::<3>().as_bytes().to_owned();
        b.iter(|| assert_eq!(part_one(test::black_box(&bs)), 2572));
    }
/*
    #[bench]
    fn p2bench(b: &mut test::Bencher) {
        let input = common::read_file::<2>();
        let bs = parse(input.as_bytes());
        b.iter(|| assert_eq!(part_two(test::black_box(&bs)), 3842356));
    }*/
}