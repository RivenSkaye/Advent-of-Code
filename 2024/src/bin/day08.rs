#![feature(test)]
extern crate test;

use std::collections::HashSet;

use aoc2024::common;

#[derive(Clone, Copy, Debug)]
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
    #[inline]
    /// gets, in order, the horizontal and vertical distance between two points
    fn distance(&self, a: usize, b: usize) -> (usize, usize) {
        (
            (a % self.line_length).abs_diff(b % self.line_length),
            (a / self.line_length).abs_diff(b / self.line_length),
        )
    }
}

impl From<&[u8]> for Grid {
    fn from(value: &[u8]) -> Self {
        Self {
            // The ordering is important or it'll complain about a move before the borrow in iter...
            // Sometimes Rust is stupid
            line_length: value.iter().position(|c| b'\n'.eq(c)).unwrap(),
            inner: value
                .iter()
                .filter_map(|c| b'\n'.ne(c).then_some(*c))
                .collect(),
        }
    }
}

fn find_antinodes_p1(grid: &Grid, current: usize, container: &mut HashSet<usize>) {
    let label = grid.inner[current];
    let xpos = grid.distance(0, current).0;
    for (val, pos) in grid.inner[(current + 1)..].iter().zip((current + 1)..) {
        if label.eq(val) {
            let (xdist, ydist) = grid.distance(current, pos);
            let nextx = grid.distance(0, pos).0;
            // note: we only need to check left/right because we only check spaces
            // later in the grid. Or for the 1D Vec, we only check higher indices
            let xdir = if nextx < xpos {
                Direction::LEFT
            } else {
                Direction::RIGHT
            };
            match xdir {
                Direction::LEFT => {
                    let jmpy = ydist * grid.line_length;
                    let jmp = jmpy - xdist;
                    if jmp <= current && (xpos + xdist) < grid.line_length {
                        container.insert(current - jmp);
                    }
                    if (pos + jmp) < grid.inner.len() && nextx >= xdist {
                        container.insert(pos + jmp);
                    }
                }
                Direction::RIGHT => {
                    let jmpy = ydist * grid.line_length;
                    let jmp = jmpy + xdist;
                    if jmp <= current && xpos >= xdist {
                        container.insert(current - jmp);
                    }
                    if (pos + jmp) < grid.inner.len() && (nextx + xdist) < grid.line_length {
                        container.insert(pos + jmp);
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

pub fn part_one(grid: &Grid) -> usize {
    let mut container = HashSet::new();
    for pos in 0..grid.inner.len() {
        if grid.inner[pos] == b'.' {
            continue;
        }
        find_antinodes_p1(grid, pos, &mut container);
    }
    container.len()
}

fn find_antinodes_p2(grid: &Grid, current: usize, container: &mut HashSet<usize>) {
    let label = grid.inner[current];
    let (xpos, ypos) = grid.distance(0, current);
    let max_y = grid.distance(0, grid.inner.len() - 1).1;
    for (val, pos) in grid.inner[(current + 1)..].iter().zip((current + 1)..) {
        if label.eq(val) {
            container.insert(current);
            container.insert(pos);
            let (xdist, ydist) = grid.distance(current, pos);
            let (nextx, nexty) = grid.distance(0, pos);
            let (mut backx, mut backy) = (xdist, ydist);
            let (mut fwrdx, mut fwrdy) = (xdist, ydist);
            if nextx < xpos {
                // next is down-left, so prev is up-right
                while backx < (grid.line_length - xpos) && backy <= ypos {
                    let jmp = (backy * grid.line_length) - backx;
                    container.insert(current - jmp);
                    backx += xdist;
                    backy += ydist;
                }
                while fwrdx <= nextx && fwrdy <= (max_y - nexty) {
                    let jmp = (fwrdy * grid.line_length) - fwrdx;
                    container.insert(pos + jmp);
                    fwrdx += xdist;
                    fwrdy += ydist;
                }
            } else {
                // next is down-right, so prev is up-left
                while backx <= xpos && backy <= ypos {
                    let jmp = (backy * grid.line_length) + backx;
                    container.insert(current - jmp);
                    backx += xdist;
                    backy += ydist;
                }
                while fwrdx < (grid.line_length - nextx) && fwrdy <= (max_y - nexty) {
                    let jmp = (fwrdy * grid.line_length) + fwrdx;
                    container.insert(pos + jmp);
                    fwrdx += xdist;
                    fwrdy += ydist;
                }
            }
        }
    }
}

pub fn part_two(grid: &Grid) -> usize {
    let mut container = HashSet::new();
    for pos in 0..grid.inner.len() {
        if grid.inner[pos] == b'.' {
            continue;
        }
        container.insert(pos);
        find_antinodes_p2(grid, pos, &mut container);
    }
    container.len()
}

pub fn main() {
    let input = common::read_file::<8>();
    let parsed = Grid::from(input.as_slice());
    println!("{}", part_one(&parsed));
    println!("{}", part_two(&parsed));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file::<8>();
        let parsed = Grid::from(input.as_slice());
        b.iter(|| {
            assert_eq!(
                Grid::from(test::black_box(input.as_slice())).line_length,
                parsed.line_length
            )
        })
    }
    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_file::<8>();
        let parsed = Grid::from(input.as_slice());
        b.iter(|| assert_eq!(part_one(test::black_box(&parsed)), 214))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_file::<8>();
        let parsed = Grid::from(input.as_slice());
        b.iter(|| assert_eq!(part_two(test::black_box(&parsed)), 809))
    }
}
