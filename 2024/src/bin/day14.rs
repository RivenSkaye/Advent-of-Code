#![feature(test)]
extern crate test;

use aoc2024::common;

#[cfg(debug_assertions)]
const HEIGHT: isize = 7;
#[cfg(debug_assertions)]
const WIDTH: isize = 11;

#[cfg(not(debug_assertions))]
const HEIGHT: isize = 101;
#[cfg(not(debug_assertions))]
const WIDTH: isize = 103;

struct Position {
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
        // 1: We only need to teleport if we go OVER the constant ...
        let next = self.x + self.xjmp;
        if next > WIDTH {
            next - WIDTH
        }
        // 2: We can leverage the negative offset as our distance post teleport
        else if next < 1 {
            WIDTH - next
        } else {
            next
        }
    }
    #[inline(always)]
    fn vjmp(&self) -> isize {
        let next = self.x + self.xjmp;
        if next > HEIGHT {
            next - HEIGHT
        } else if next < 1 {
            HEIGHT - next
        } else {
            next
        }
    }
}

pub fn parse(input: &[u8]) -> Vec<Position> {
    input
        .split(|chr| b'\n'.eq(chr))
        .map(|line| {
            let (poschunk, velchunk) = line.split_once(pred);
        })
        .collect()
}

pub fn main() {}
