#![feature(slice_split_once, test)]
extern crate test;

use aoc2024::common;

#[cfg(debug_assertions)]
const HEIGHT: isize = 7;
#[cfg(debug_assertions)]
const WIDTH: isize = 11;

#[cfg(not(debug_assertions))]
const HEIGHT: isize = 103;
#[cfg(not(debug_assertions))]
const WIDTH: isize = 101;

// Integer division truncates off the middle row/column
// So abusing that we can pass the bots into quarters by
// only checking lt and gt on either axis, ignoring eq.
// And thanks to width and height being given constants,
// we know they're always odd numbers! A freebie!
const H_SPLIT: isize = WIDTH / 2;
const V_SPLIT: isize = HEIGHT / 2;
// Post-mortem: input is zero-indexed, which is mean.

#[derive(Clone, Copy)]
pub struct Position {
    x: isize,
    y: isize,
    xjmp: isize,
    yjmp: isize,
}

impl Position {
    fn jump(&self) -> Self {
        Self {
            x: self.hjmp(),
            y: self.vjmp(),
            xjmp: self.xjmp,
            yjmp: self.yjmp,
        }
    }

    #[inline(always)]
    fn hjmp(&self) -> isize {
        // By indexing positions from 1, we can equate 0 and MAX.
        // This allows us to leverage a couple things.
        // 1: We only need to teleport if we go OVER the constant and
        // 2: We can leverage the negative offset as our distance post teleport
        match self.x + self.xjmp {
            n if n < 0 => WIDTH + n, // the + is cursed, but n is negative
            n if n >= WIDTH => n - WIDTH,
            n => n,
        }
    }
    #[inline(always)]
    fn vjmp(&self) -> isize {
        match self.y + self.yjmp {
            n if n < 0 => HEIGHT + n, // the + is cursed, but n is negative
            n if n >= HEIGHT => n - HEIGHT,
            n => n,
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

pub fn parse(input: &[u8]) -> Vec<Position> {
    input
        .split(|chr| b'\n'.eq(chr))
        .map(|line| {
            let (poschunk, velchunk) = line
                .split_once(|chr| b' '.eq(chr))
                // Yank off "p=" and "v="
                .map(|(p, v)| (p.split_at(2).1, v.split_at(2).1))
                .unwrap();
            let (xpos, ypos) = poschunk
                .split_once(|chr| b','.eq(chr))
                .map(|(x, y)| (common::stosi(x), common::stosi(y)))
                .unwrap();
            let (xvel, yvel) = velchunk
                .split_once(|chr| b','.eq(chr))
                .map(|(x, y)| (common::stosi(x), common::stosi(y)))
                .unwrap();
            Position {
                x: xpos,
                y: ypos,
                xjmp: xvel,
                yjmp: yvel,
            }
        })
        .collect()
}

pub fn part_one(positions: &[Position]) -> isize {
    let after: Vec<_> = positions
        .iter()
        .map(|robot| {
            let mut next_robo = robot.clone();
            for _ in 0..100 {
                next_robo = next_robo.jump();
            }
            next_robo
        })
        .collect();
    // top-left, top-right, bottom-left, bottom-right
    let [mut q1, mut q2, mut q3, mut q4] = [0; 4];
    for position in after {
        if position.x == H_SPLIT || position.y == V_SPLIT {
            continue;
        }
        match (position.y < V_SPLIT, position.x < H_SPLIT) {
            (true, true) => q1 += 1,
            (true, false) => q2 += 1,
            (false, true) => q3 += 1,
            (false, false) => q4 += 1,
        }
    }
    q1 * q2 * q3 * q4
}

pub fn part_two(parsed: &[Position]) -> isize {
    let mut robots = parsed.to_vec();
    for _ in 1..5000 {
        for idx in 0..robots.len() {
            robots[idx] = robots[idx].jump();
        }
    }
    for step in 5000.. {
        for idx in 0..robots.len() {
            robots[idx] = robots[idx].jump();
        }
        if robots.iter().map(|r| (r.x, r.y)).any(|(x, y)| {
            (0..5)
                .flat_map(|width| [(x - width, y), (x + width, y)])
                .all(|(x, y)| {
                    robots.contains(&Position {
                        x,
                        y,
                        xjmp: 0,
                        yjmp: 0,
                    })
                })
        }) {
            return step;
        }
    }
    unreachable!()
}

pub fn main() {
    let input = common::read_file::<14>();
    let parsed = parse(&input);
    println!("{}", part_one(&parsed));
    println!("{}", part_two(&parsed));
}
