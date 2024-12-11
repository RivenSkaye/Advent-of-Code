#![feature(vec_push_within_capacity, test)]
extern crate test;

use std::usize;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn rev(&self) -> Self {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
        }
    }
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
        if fval.abs_diff(nval) > 1 || fval > nval {
            None
        } else {
            nextpos.into()
        }
    }
}

pub fn parse(input: &[u8]) -> (Grid, Vec<usize>) {
    let len = input.iter().position(|c| b'\n'.eq(c)).unwrap();
    //over-allocate and make the assumption 1 in 5 numbers is a zero. This cound also still includes newlines.
    let mut tailheads = Vec::with_capacity(input.len() / 5);
    (
        Grid {
            line_length: len,
            inner: input
                .iter()
                .filter(|c| b'\n'.ne(c))
                .enumerate()
                .map(|(idx, p)| {
                    if b'0'.eq(p) {
                        tailheads.push_within_capacity(idx).unwrap();
                    }
                    *p
                })
                .collect(),
        },
        tailheads,
    )
}

fn walk_from(grid: Grid, position: usize, start_dir: Direction) -> usize {
    const DIRECTIONS: [Direction; 4] = [
        Direction::UP,
        Direction::DOWN,
        Direction::LEFT,
        Direction::RIGHT,
    ];
    DIRECTIONS.iter().filter(|&&d| d != start_dir.rev()).map({
        let next;
    })
}

pub fn part_one(grid: Grid, heads: &[u8]) -> usize {}

pub fn main() {}
