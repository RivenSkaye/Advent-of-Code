#![feature(test)]
extern crate test;

use std::collections::BTreeSet;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use aoc2023::common;

// Literally just for the benches
#[derive(Debug, PartialEq)]
pub struct Point(usize, usize, usize, usize);

impl Point {
    /// This is all about manhattan distances
    pub fn distance<const EXPANSION: usize>(&self, other: &Self) -> usize {
        let sx = self.1 + (self.3 * EXPANSION) - self.3;
        let sy = self.0 + (self.2 * EXPANSION) - self.2;
        let ox = other.1 + (other.3 * EXPANSION) - other.3;
        let oy = other.0 + (other.2 * EXPANSION) - other.2;
        sx.abs_diff(ox) + sy.abs_diff(oy)
    }
}

pub fn parse(input: &[u8]) -> Vec<Point> {
    let mut found_x = BTreeSet::new();
    let mut found_y = BTreeSet::new();
    // Use 1 to compensate for the newline we're including in the line widths
    let mut width = 1;
    // 1-indexed lines because of initial code and I'm afraid of breaking a working solution
    let mut height = 1;
    let mut points = input
        .iter()
        .enumerate()
        .filter_map(|(pos, point)| {
            match point {
                b'\n' => {
                    if width == 1 {
                        width += pos
                    }
                    height += 1
                }
                b'#' => {
                    let x = pos - ((height - 1) * width);
                    found_x.insert(x);
                    found_y.insert(height);
                    return Some(Point(height, x, 0, 0));
                }
                _ => (),
            }
            None
        })
        .collect::<Vec<_>>();
    for x in (0..=width).rev() {
        if !found_x.contains(&x) {
            points.iter_mut().for_each(|point| {
                if point.1 > x {
                    point.3 += 1
                }
            })
        }
    }
    for y in (1..height).rev() {
        if !found_y.contains(&y) {
            points.iter_mut().for_each(|point| {
                if point.0 > y {
                    point.2 += 1
                }
            })
        }
    }
    points
}

pub fn part_one(points: &[Point]) -> usize {
    let len = points.len();
    points[..len]
        .iter()
        .zip(1..)
        .fold(0, |last, (point, next)| {
            last + (next..len).fold(0, |l, i| l + points[i].distance::<2>(point))
        })
}

pub fn part_two(points: &[Point]) -> usize {
    let len = points.len();
    points[..len]
        .iter()
        .zip(1..)
        .fold(0, |last, (point, next)| {
            last + (next..len).fold(0, |l, i| l + points[i].distance::<1000000>(point))
        })
}

pub fn main() {
    let input = common::read_file::<11>();
    let parsed = parse(&input);
    println!("{}", part_one(&parsed));
    println!("{}", part_two(&parsed));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file::<11>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(parse(test::black_box(&input)), parsed))
    }
    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_file::<11>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part_one(test::black_box(&parsed)), 9556712))
    }
    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_file::<11>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part_two(test::black_box(&parsed)), 678626199476))
    }
}
