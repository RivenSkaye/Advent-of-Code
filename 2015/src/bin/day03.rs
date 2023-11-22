#![feature(test)]
extern crate test;

use aoc2015::common;

struct Point {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Point {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<(Point, u8)> for Point {
    fn from(value: (Point, u8)) -> Self {
        match value.1 {
            b'<' => (value.0.x - 1, value.0.y),
            b'^' => (value.0.x, value.0.y + 1),
            b'>' => (value.0.x + 1, value.0.y),
            b'v' => (value.0.x, value.0.y + 1),
            _ => unreachable!(),
        }
        .into()
    }
}

fn part_one(input: &[u8]) -> isize {
    0
}

fn main() {}
