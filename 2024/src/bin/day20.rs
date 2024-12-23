#![feature(test)]
extern crate test;

use aoc2024::{
    common,
    grids::{Direction, FlatGrid as Grid},
};
use std::collections::HashMap;

#[cfg(debug_assertions)]
const SHAVE: usize = 50;
#[cfg(not(debug_assertions))]
const SHAVE: usize = 100;

pub fn parse(data: &[u8]) -> (Direction, usize, usize, usize, Grid) {
    let g: Grid = data.into();
    let start = g.inner.iter().position(|tile| b'S'.eq(tile)).unwrap();
    let end = g.inner.iter().position(|tile| b'E'.eq(tile)).unwrap();
    let start_dir = [
        Direction::UP,
        Direction::DOWN,
        Direction::LEFT,
        Direction::RIGHT,
    ]
    .into_iter()
    .find(|&dir| g.inner[g.unbounded_step(start, dir)] != b'#')
    .unwrap();
    let uncheated = get_base(&g, start, end, 0, start_dir);
    (start_dir, uncheated, start, end, g)
}

fn get_base(grid: &Grid, start: usize, end: usize, steps: usize, dir: Direction) -> usize {
    if start == end {
        return steps;
    }
    for dir in [dir, dir.turn_clock(), dir.turn_counter()] {
        let next = grid.unbounded_step(start, dir);
        if grid.inner[next] != b'#' {
            return get_base(grid, next, end, steps + 1, dir);
        }
    }
    unreachable!()
}

fn step_cheat(
    grid: &Grid,
    start: usize,
    end: usize,
    steps: usize,
    to_beat: usize,
    dirs: &[Direction],
    cheated: bool,
    paths: &mut Vec<usize>,
) {
    if steps + SHAVE > to_beat {
        return;
    }
    if start == end {
        paths.push(steps);
    }
    dirs.into_iter().for_each(|&dir| {
        if let Some(next) = grid.bounded_step(start, dir) {
            if grid.inner[next] != b'#' {
                return step_cheat(
                    grid,
                    next,
                    end,
                    steps + 1,
                    to_beat,
                    &[dir, dir.turn_clock(), dir.turn_counter()],
                    cheated,
                    paths,
                );
            } else if !cheated {
                return step_cheat(
                    grid,
                    next,
                    end,
                    steps + 1,
                    to_beat,
                    &[dir, dir.turn_clock(), dir.turn_counter()],
                    true,
                    paths,
                );
            }
        }
    });
}

pub fn part_one(
    grid: &Grid,
    start_dir: Direction,
    start: usize,
    end: usize,
    uncheated: usize,
) -> usize {
    let mut cheated = Vec::with_capacity(grid.inner.len());
    step_cheat(
        grid,
        start,
        end,
        0,
        uncheated,
        &[start_dir, start_dir.turn_clock(), start_dir.turn_counter()],
        false,
        &mut cheated,
    );
    cheated.len()
}

fn long_cheat(
    grid: &Grid,
    start: usize,
    end: usize,
    steps: usize,
    to_beat: usize,
    dirs: &[Direction],
    active_cheat: Option<(usize, usize)>,
    cheat_steps: u8,
    paths: &mut HashMap<(usize, usize), usize>,
) {
    if steps + SHAVE > to_beat {
        return;
    }
    if start == end {
        paths.insert(active_cheat.unwrap(), steps);
    }
    dirs.into_iter().for_each(|&dir| {
        if let Some(next) = grid.bounded_step(start, dir) {
            if grid.inner[next] != b'#' {
                let next_ac = if let Some(ac) = active_cheat {
                    Some((ac.0, next))
                } else {
                    active_cheat
                };
                return long_cheat(
                    grid,
                    next,
                    end,
                    steps + 1,
                    to_beat,
                    &[dir, dir.turn_clock(), dir.turn_counter()],
                    next_ac,
                    cheat_steps - 1,
                    paths,
                );
            } else if cheat_steps > 0 {
                let next_ac = if active_cheat.is_none() {
                    Some((next, next))
                } else {
                    active_cheat
                };
                return long_cheat(
                    grid,
                    next,
                    end,
                    steps + 1,
                    to_beat,
                    &[dir, dir.turn_clock(), dir.turn_counter()],
                    next_ac,
                    cheat_steps - 1,
                    paths,
                );
            }
        }
    });
}

pub fn part_two(grid: &Grid, start: usize, end: usize, uncheated: usize) -> usize {
    let mut cheated = HashMap::with_capacity(grid.inner.len());
    long_cheat(
        grid,
        start,
        end,
        0,
        uncheated,
        &[
            Direction::UP,
            Direction::DOWN,
            Direction::LEFT,
            Direction::RIGHT,
        ],
        None,
        20,
        &mut cheated,
    );
    cheated.len()
}

pub fn main() {
    let data = common::read_file::<20>();
    let (start_dir, uncheated, start, end, grid) = parse(&data);
    println!("{}", part_one(&grid, start_dir, start, end, uncheated)); // Test: 1 path
    println!("{}", part_two(&grid, start, end, uncheated)); // Test: 285 paths
}
