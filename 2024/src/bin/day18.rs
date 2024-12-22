#![feature(slice_split_once, test)]
extern crate test;

use pathfinding::prelude::dijkstra;

use aoc2024::common;

#[cfg(debug_assertions)]
const WIDTH: usize = 6;
#[cfg(not(debug_assertions))]
const WIDTH: usize = 70;

#[cfg(debug_assertions)]
const HEIGHT: usize = 6;
#[cfg(not(debug_assertions))]
const HEIGHT: usize = 70;

#[cfg(debug_assertions)]
const TTL: usize = 12;
#[cfg(not(debug_assertions))]
const TTL: usize = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Position {
    x: usize,
    y: usize,
}

const GOAL: Position = Position {
    x: WIDTH,
    y: HEIGHT,
};

impl Position {
    fn left(&self) -> Option<Self> {
        self.x.checked_sub(1).map(|x| Self { x, y: self.y })
    }
    fn right(&self) -> Option<Self> {
        (self.x < WIDTH).then(|| Self {
            x: self.x + 1,
            y: self.y,
        })
    }
    fn up(&self) -> Option<Self> {
        self.y.checked_sub(1).map(|y| Self { x: self.x, y })
    }
    fn down(&self) -> Option<Self> {
        (self.y < HEIGHT).then(|| Self {
            x: self.x,
            y: self.y + 1,
        })
    }

    pub fn successors(&self) -> Vec<Self> {
        [self.left(), self.right(), self.up(), self.down()]
            .into_iter()
            .filter_map(|p| p)
            .collect()
    }

    pub fn printable(&self) -> String {
        format!("{},{}", self.x, self.y)
    }
}

pub fn parse(data: &[u8]) -> Vec<Position> {
    data.split(|chr| b'\n'.eq(chr))
        .map(|line| {
            line.split_once(|chr| b','.eq(chr))
                .map(|(x, y)| Position {
                    x: common::stoi(x) as usize,
                    y: common::stoi(y) as usize,
                })
                .unwrap()
        })
        .collect()
}

pub fn part_one(corrupted: &[Position]) -> usize {
    dijkstra(
        &Position { x: 0, y: 0 },
        |p| {
            p.successors()
                .iter()
                .filter_map(|&pos| (!corrupted.contains(&pos)).then_some((pos, 1)))
                .collect::<Vec<_>>()
        },
        |p| GOAL.eq(p),
    )
    .unwrap()
    .1
}

pub fn part_two(corrupted: &[Position]) -> String {
    // shortcut: p1 taught us a route is guaranteed until TTL
    for next in TTL.. {
        if dijkstra(
            &Position { x: 0, y: 0 },
            |p| {
                p.successors()
                    .iter()
                    .filter_map(|&pos| (!corrupted[..=next].contains(&pos)).then_some((pos, 1)))
                    .collect::<Vec<_>>()
            },
            |p| GOAL.eq(p),
        )
        .is_none()
        {
            return corrupted[next].printable();
        }
    }
    unreachable!()
}

pub fn main() {
    let data = common::read_file::<18>();
    let corruption = parse(&data);
    println!("{}", part_one(&corruption[..TTL]));
    println!("{}", part_two(&corruption));
}
