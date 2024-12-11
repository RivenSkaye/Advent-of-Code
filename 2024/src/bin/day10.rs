#![feature(vec_push_within_capacity, test)]
extern crate test;

use std::usize;

use aoc2024::common;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone)]
pub struct Grid {
    inner: Vec<u8>,
    line_length: usize,
}

impl Grid {
    fn step(&self, from: usize, dir: Direction) -> Option<usize> {
        let nextpos = match (from, dir) {
            (n, Direction::UP) => (n > self.line_length).then(|| n - self.line_length),
            (n, Direction::LEFT) => (n % self.line_length > 0).then(|| n - 1),
            (n, Direction::RIGHT) => ((n + 1) % self.line_length > 0).then(|| n + 1),
            (n, Direction::DOWN) => {
                (n + self.line_length < self.inner.len()).then(|| n + self.line_length)
            }
        }?;
        let nval = self.inner[nextpos];
        let fval = self.inner[from];
        (fval.abs_diff(nval) == 1 && fval < nval).then_some(nextpos)
    }
}

pub fn parse(input: &[u8]) -> Grid {
    Grid {
        line_length: input.iter().position(|c| b'\n'.eq(c)).unwrap(),
        inner: input
            .iter()
            .filter_map(|c| b'\n'.ne(c).then(|| *c))
            .collect(),
    }
}

fn walk_from(grid: &Grid, position: usize, ends: &mut Vec<usize>) {
    const DIRECTIONS: [Direction; 4] = [
        Direction::UP,
        Direction::DOWN,
        Direction::LEFT,
        Direction::RIGHT,
    ];
    if grid.inner[position] == b'9' {
        return ends.push_within_capacity(position).unwrap();
    }
    DIRECTIONS
        .iter()
        .filter_map(|&dir| grid.step(position, dir))
        .for_each(|position| walk_from(grid, position, ends));
}

pub fn part_one(grid: &Grid) -> usize {
    grid.inner
        .iter()
        .enumerate()
        .filter_map(|(idx, val)| {
            b'0'.eq(val).then(|| {
                let mut ends = Vec::with_capacity(grid.inner.len());
                walk_from(grid, idx, &mut ends);
                ends.len()
            })
        })
        .sum()
}

pub fn main() {
    let input = common::read_file::<10>();
    let parsed = parse(&input);
    println!("{}", part_one(&parsed));
}
