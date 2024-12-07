#![feature(test)]
extern crate test;

use std::collections::HashSet;

use aoc2024::common;

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum Direction {
    UP = b'^',
    DOWN = b'v',
    LEFT = b'<',
    RIGHT = b'>',
}

impl Direction {
    pub fn turn(self) -> Self {
        match self {
            Self::UP => Self::RIGHT,
            Self::RIGHT => Self::DOWN,
            Self::DOWN => Self::LEFT,
            Self::LEFT => Self::UP,
        }
    }
}

#[derive(Clone)]
pub struct Grid {
    inner: Vec<u8>,
    line_length: usize,
}

impl From<Vec<u8>> for Grid {
    fn from(value: Vec<u8>) -> Self {
        Self {
            // The ordering is important or it'll complain about a move before the borrow in iter...
            // Sometimes Rust is stupid
            line_length: value.iter().position(|c| b'\n'.eq(c)).unwrap() + 1,
            inner: value,
        }
    }
}

impl Grid {
    fn step(&self, current: usize, dir: Direction) -> Option<(usize, Direction)> {
        #[inline(always)]
        fn _can_step(_self: &Grid, idx: usize) -> bool {
            idx < _self.inner.len() && _self.inner[idx] != b'#'
        }
        match dir {
            Direction::UP if current >= self.line_length => {
                let next = current - self.line_length;
                _can_step(self, next)
                    .then_some((next, dir))
                    .or_else(|| self.step(current, dir.turn()))
            }
            Direction::RIGHT if ((current + 2) % self.line_length) > 0 => {
                let next = current + 1;
                _can_step(self, next)
                    .then_some((next, dir))
                    .or_else(|| self.step(current, dir.turn()))
            }
            Direction::DOWN if current + self.line_length < self.inner.len() => {
                let next = current + self.line_length;
                _can_step(self, next)
                    .then_some((next, dir))
                    .or_else(|| self.step(current, dir.turn()))
            }
            Direction::LEFT if current % self.line_length > 0 => {
                let next = current - 1;
                _can_step(self, next)
                    .then_some((next, dir))
                    .or_else(|| self.step(current, dir.turn()))
            }
            _ => None,
        }
    }

    fn is_cycle(&self, mut current: usize, moves: &mut HashSet<usize>) -> bool {
        /// Szudzik's elegant pairing function: http://szudzik.com/ElegantPairing.pdf
        #[inline(always)]
        fn _pair(curr: usize, dir: usize) -> usize {
            curr * curr + curr + dir
        }
        let mut dir = Direction::UP;
        while let Some((step, turned)) = self.step(current, dir) {
            // We left shift by 7 because Directions are never more than 118.
            // This shift ensures step > turned for the pairing function.
            if moves.contains(&_pair(step << 7, turned as usize)) {
                moves.clear();
                return true;
            }
            moves.insert(_pair(step << 7, turned as usize));
            dir = turned;
            current = step;
        }
        moves.clear();
        false
    }
}

#[inline(always)]
pub fn parse(input: &[u8]) -> usize {
    input.iter().position(|u| b'^'.eq(u)).unwrap()
}

pub fn part_one(grid: &mut Grid, mut position: usize) -> usize {
    let mut dir = Direction::UP;
    while let Some(step) = grid.step(position, dir) {
        position = step.0;
        dir = step.1;
        grid.inner[position] = b'x';
    }
    grid.inner.iter().filter(|c| b'x'.eq(c)).count() + 1
}

pub fn part_two(grid: &mut Grid, start_position: usize) -> usize {
    let mut dir = Direction::UP;
    let mut position = start_position;
    let mut checks = HashSet::with_capacity(5000);
    while let Some(step) = grid.step(position, dir) {
        position = step.0;
        dir = step.1;
        checks.insert(step.0);
    }
    let mut moves = HashSet::with_capacity(checks.len() * 2);
    checks
        .iter()
        .filter(|&&i| {
            grid.inner[i] = b'#';
            let res = grid.is_cycle(start_position, &mut moves);
            grid.inner[i] = b'x';
            res
        })
        .count()
}

pub fn main() {
    let input = common::read_file::<6>();
    let pos = parse(&input);
    let parsed = Grid::from(input);
    println!("{}", part_one(&mut parsed.clone(), pos));
    println!("{}", part_two(&mut parsed.clone(), pos))
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file::<6>();
        let parsed = parse(&input);
        let refgrid = Grid::from(input.clone());
        b.iter(|| {
            assert_eq!(parse(test::black_box(&input)), parsed);
            assert_eq!(
                refgrid.line_length,
                test::black_box(input.iter().position(|c| b'\n'.eq(c)).unwrap() + 1)
            );
        })
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_file::<6>();
        let pos = parse(&input);
        let mut parsed = Grid::from(input);
        b.iter(|| {
            assert_eq!(
                part_one(test::black_box(&mut parsed), test::black_box(pos)),
                4647
            )
        })
    }

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
    }
}
