#![feature(test)]
extern crate test;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use std::mem::transmute;

/// Safely transmute valid map entries (u8) to Pipes, see
/// [faster than lime](https://fasterthanli.me/articles/peeking-inside-a-rust-enum)
/// e.g. let pipe = unsafe { transmute(b'|') };
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Pipe {
    UpDown = b'|',    // xConnec, y +/- 1
    LeftRight = b'-', // x +/- 1, y
    DownRight = b'L', // x + 1, y || x, y - 1
    DownLeft = b'J',  // x - 1, y || x, y - 1
    UpLeft = b'7',    // x, y + 1 || x - 1, y
    UpRight = b'F',   // x + 1, y || x, y + 1
    NoPipe = b'.',    // Can't step
    Start = b'S',     // Wildcard
}

#[cfg(debug_assertions)]
const MAP_DIMENSIONS: usize = 5;
#[cfg(not(debug_assertions))]
const MAP_DIMENSIONS: usize = 140;

pub struct Point {
    x: usize,
    y: usize,
}

pub type Map = [[Pipe; MAP_DIMENSIONS]; MAP_DIMENSIONS];

pub fn parse(input: &[u8]) -> (Map, (usize, usize)) {
    let mut map: Map = [[0; MAP_DIMENSIONS]; MAP_DIMENSIONS];
    let mut start = (0, 0);
    input
        .split(|c| b'\n'.eq(c))
        .enumerate()
        .for_each(|(y, line)| {
            line.iter().enumerate().for_each(|(x, chr)| {
                if b'S'.eq(chr) {
                    start = (y, x);
                }
                // SAFETY: see Pipe
                map[y][x] = unsafe { transmute(*chr) };
            })
        });
    (map, start)
}

pub fn main() {}
