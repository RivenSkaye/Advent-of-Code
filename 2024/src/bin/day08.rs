#![feature(let_chains, test)]
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
            // note: we only need to check left/right because we only check spaces
            // later in the grid. Or for the 1D Vec, we only check higher indices
            let xdir = if grid.distance(0, pos).0 < xpos {
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
                    if (pos + jmp) < grid.inner.len() && xpos >= xdist {
                        container.insert(pos + jmp);
                    }
                }
                Direction::RIGHT => {
                    let jmpy = ydist * grid.line_length;
                    let jmp = jmpy + xdist;
                    if jmp <= current && xpos >= xdist {
                        container.insert(current - jmp);
                    }
                    if (pos + jmp) < grid.inner.len() && (xpos + xdist) < grid.line_length {
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
    for i in 0..grid.inner.len() {
        if i % grid.line_length == 0 {
            println!("")
        }
        print!(
            "{}",
            if container.contains(&i) && grid.inner[i] == b'.' {
                String::from("#")
            } else {
                String::from_utf8_lossy(&grid.inner[i..=i]).into()
            }
        )
    }
    println!("\n{container:?}");
    container.len()
}

fn find_antinodes_p2(grid: &Grid, current: usize, container: &mut HashSet<usize>) {
    let label = grid.inner[current];
    let xpos = grid.distance(0, current).0;
    for (val, pos) in grid.inner[(current + 1)..].iter().zip((current + 1)..) {
        if label.eq(val) {
            let (xdist, ydist) = grid.distance(current, pos);
            // note: we only need to check left/right because we only check spaces
            // later in the grid. Or for the 1D Vec, we only check higher indices
            let xdir = if pos % grid.line_length < xpos {
                Direction::LEFT
            } else {
                Direction::RIGHT
            };
            let mut back = true;
            let mut forward = true;
            let mut jmpy = 0;
            let mut jmpx = 0;
            match xdir {
                Direction::LEFT => {
                    while back || forward {
                        jmpy += ydist * grid.line_length;
                        jmpx += xdist;
                        let jmp = jmpy - jmpx;
                        if back && jmp <= current && (grid.line_length % xpos) >= jmpx {
                            container.insert(current - jmp);
                        } else {
                            back = false;
                        }
                        if forward && (pos + jmp) < grid.inner.len() && xpos >= jmpx {
                            container.insert(pos + jmp);
                        } else {
                            forward = false;
                        }
                    }
                }
                Direction::RIGHT => {
                    while back || forward {
                        jmpy += ydist * grid.line_length;
                        jmpx += xdist;
                        let jmp = jmpy - jmpx;
                        if back && jmp <= current && xpos >= jmpx {
                            container.insert(current - jmp);
                        } else {
                            back = false;
                        }
                        if forward
                            && (pos + jmp) < grid.inner.len()
                            && (grid.line_length % xpos) >= jmpx
                        {
                            container.insert(pos + jmp);
                        } else {
                            forward = false;
                        }
                    }
                }
                _ => unreachable!(),
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
    for i in 0..grid.inner.len() {
        if i % grid.line_length == 0 {
            println!("")
        }
        print!(
            "{}",
            if container.contains(&i) && grid.inner[i] == b'.' {
                String::from("#")
            } else {
                String::from_utf8_lossy(&grid.inner[i..=i]).into()
            }
        )
    }
    println!("");
    container.len()
}

pub fn main() {
    let input = common::read_file::<8>();
    let parsed = Grid::from(input.as_slice());
    println!("{}", part_one(&parsed));
    println!("{}", part_two(&parsed));
}
