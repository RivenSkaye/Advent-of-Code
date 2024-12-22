#![feature(slice_split_once, test)]
extern crate test;

use std::collections::HashMap;

use aoc2024::{
    common,
    grids::{Direction, FlatGrid as Grid},
};

#[cfg(debug_assertions)]
const WIDTH: usize = 7; // 0 through 6
#[cfg(not(debug_assertions))]
const WIDTH: usize = 71; // 0 through 70

#[cfg(debug_assertions)]
const HEIGHT: usize = 7; // 0 through 6
#[cfg(not(debug_assertions))]
const HEIGHT: usize = 71; // 0 through 70

#[cfg(debug_assertions)]
const TTL: usize = 12;
#[cfg(not(debug_assertions))]
const TTL: usize = 1024;

#[derive(Debug)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn to_flat_pos(&self) -> usize {
        (self.y * WIDTH) + self.x
    }
}

pub fn parse(data: &[u8]) -> (Grid, Vec<Position>) {
    (
        Grid {
            inner: [b'.'; WIDTH * HEIGHT].into(),
            line_length: WIDTH,
        },
        data.split(|chr| b'\n'.eq(chr))
            .map(|line| {
                line.split_once(|chr| b','.eq(chr))
                    .map(|(x, y)| Position {
                        x: common::stoi(x),
                        y: common::stoi(y),
                    })
                    .unwrap()
            })
            .collect(),
    )
}

fn walk(
    grid: &mut Grid,
    current: usize,
    dir: Direction,
    visited: &mut HashMap<(usize, Direction), usize>,
    steps: usize,
    max_steps: &mut usize,
) -> Option<usize> {
    if current == grid.inner.len() - 1 {
        if steps.lt(max_steps) {
            *max_steps = steps;
        }
        return Some(steps);
    }
    visited.insert((current, dir), steps);

    [dir.turn_clock(), dir, dir.turn_counter()]
        .iter()
        .filter_map(|&dir| {
            if let Some(next) = grid.bounded_step(current, dir) {
                if grid.inner[next] == b'#'
                    || steps.ge(max_steps)
                    || visited.get(&(next, dir)).is_some_and(|s| steps.ge(s))
                {
                    None
                } else {
                    walk(grid, next, dir, visited, steps + 1, max_steps)
                }
            } else {
                None
            }
        })
        .min()
}

pub fn part_one(grid: &mut Grid, corrupt: &[Position]) -> usize {
    for pos in &corrupt[..TTL] {
        grid.inner[pos.to_flat_pos()] = b'#';
    }
    // Manual solve puts me in the low 300s
    let mut max = 350;
    walk(
        grid,
        0,
        Direction::RIGHT,
        &mut HashMap::with_capacity(grid.inner.len()),
        0,
        &mut max,
    )
    .unwrap();
    max
}

pub fn main() {
    let data = common::read_file::<18>();
    let (mut grid, corruption) = parse(&data);
    println!("{}", part_one(&mut grid, &corruption));
}
