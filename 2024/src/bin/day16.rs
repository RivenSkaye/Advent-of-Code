#![feature(test)]
extern crate test;

use std::collections::HashMap;

use aoc2024::{
    common,
    grids::{Direction, FlatGrid as Grid},
};

#[inline(always)]
pub fn parse(input: &[u8]) -> (usize, Grid) {
    let g = Grid::from(input);
    (g.inner.iter().position(|tile| b'S'.eq(tile)).unwrap(), g)
}

fn walk(
    grid: &Grid,
    current: usize,
    dir: Direction,
    score: usize,
    visited: &mut HashMap<usize, usize>,
) -> Option<usize> {
    if grid.inner[current] == b'E' {
        return Some(score);
    }
    visited.insert(current, score);
    [dir, dir.turn_clock(), dir.turn_counter()]
        .iter()
        .filter_map(|&d| {
            let next = grid.unbounded_step(current, d);
            if grid.inner[next] == b'#' || visited.get(&next).is_some_and(|s| score.ge(s)) {
                // don't walk through walls
                return None;
            }
            let next_score = if d != dir { score + 1001 } else { score + 1 };
            walk(grid, next, d, next_score, visited)
        })
        .min()
}

pub fn part_one(grid: &Grid, start: usize) -> usize {
    let mut visited = HashMap::with_capacity(grid.inner.len());
    walk(grid, start, Direction::RIGHT, 0, &mut visited).unwrap()
}

pub fn main() {
    let data = common::read_file::<16>();
    let (start, grid) = parse(&data);
    println!("Found S at {:?}", grid.position(start));
    println!("{}", part_one(&grid, start));
}
