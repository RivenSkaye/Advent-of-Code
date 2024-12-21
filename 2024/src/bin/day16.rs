#![feature(test)]
extern crate test;

use std::{collections::HashMap, usize};

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
    visited: &mut HashMap<(usize, Direction), usize>,
    max_score: &mut usize,
) -> Option<usize> {
    if grid.inner[current] == b'E' {
        if score.lt(max_score) {
            *max_score = score
        }
        return Some(score);
    }
    visited.insert((current, dir), score);
    [dir.turn_clock(), dir, dir.turn_counter()]
        .iter()
        .filter_map(|&d| {
            let next = grid.unbounded_step(current, d);
            if grid.inner[next] == b'#'
                || score.ge(max_score)
                || visited.get(&(next, d)).is_some_and(|s| score.ge(s))
            {
                // don't walk through walls
                return None;
            }
            let next_score = if d != dir { score + 1001 } else { score + 1 };
            walk(grid, next, d, next_score, visited, max_score)
        })
        .min()
}

pub fn part_one(grid: &Grid, start: usize) -> usize {
    let mut visited = HashMap::with_capacity(grid.inner.len());
    let mut max_score = usize::MAX;
    walk(
        grid,
        start,
        Direction::RIGHT,
        0,
        &mut visited,
        &mut max_score,
    )
    .unwrap()
}

fn walk2(
    grid: &Grid,
    current: usize,
    dir: Direction,
    score: usize,
    visited: &mut HashMap<(usize, Direction), usize>,
    max_score: usize,
    on_good_path: &mut Vec<usize>,
) -> Option<usize> {
    if grid.inner[current] == b'E' {
        return Some(score);
    }
    visited.insert((current, dir), score);
    [dir.turn_clock(), dir, dir.turn_counter()]
        .iter()
        .filter_map(|&d| {
            let next = grid.unbounded_step(current, d);
            if grid.inner[next] == b'#'
                || score >= max_score
                || visited.get(&(next, d)).is_some_and(|s| score.ge(s))
            {
                // don't walk through walls
                return None;
            }
            let next_score = if d != dir { score + 1001 } else { score + 1 };
            let res = walk2(grid, next, d, next_score, visited, max_score, on_good_path);
            if res.is_some() && !on_good_path.contains(&current) {
                on_good_path.push(current);
            }
            res
        })
        .min()
}

pub fn part_two(grid: &Grid, start: usize) -> usize {
    let mut visited = HashMap::with_capacity(grid.inner.len());
    let mut max_score = usize::MAX;
    walk(
        grid,
        start,
        Direction::RIGHT,
        0,
        &mut visited,
        &mut max_score,
    );
    let mut good = Vec::with_capacity(grid.inner.len());
    walk2(
        grid,
        start,
        Direction::RIGHT,
        0,
        &mut visited,
        max_score,
        &mut good,
    );
    good.len() + 1
}

pub fn main() {
    let data = common::read_file::<16>();
    let (start, grid) = parse(&data);
    println!("{}", part_one(&grid, start));
    println!("{}", part_two(&grid, start));
}
