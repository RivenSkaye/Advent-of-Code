#![feature(array_windows, test)]
extern crate test;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use aoc2024::common;

const XMAS: &[u8; 4] = b"XMAS";
const SAMX: &[u8; 4] = b"SAMX";

/// Our parser today just tells us what jump to make to skip over a line.
pub fn parse(input: &[u8]) -> usize {
    for (chr, idx) in input.iter().zip(1..) {
        if b'\n'.eq(chr) {
            return idx;
        }
    }
    unreachable!()
}

pub fn part_one(data: &[u8], jump: usize) -> usize {
    let mut found = 0;
    for idx in 0..data.len() {
        // We only start searching from X to prevent false positives
        if b'X' != data[idx] {
            continue;
        }
        // Horizontal search forward
        if idx < data.len() - 3 && &data[idx..(idx + 4)] == XMAS {
            found += 1;
        }
        // horizontal search backward
        if idx > 3 && &data[(idx - 3)..=idx] == SAMX {
            found += 1;
        }
        // Okay we have an X still, so let's do directional searches.
        // straight down:
        if idx + (3 * jump) < data.len()
            && ([
                data[idx],
                data[idx + jump],
                data[idx + (2 * jump)],
                data[idx + (3 * jump)],
            ]
            .eq(XMAS)
                || [
                    data[idx],
                    data[idx + jump],
                    data[idx + (2 * jump)],
                    data[idx + (3 * jump)],
                ]
                .eq(SAMX))
        {
            found += 1;
        }
        // straight up:
        if idx >= (3 * jump)
            && ([
                data[idx],
                data[idx - jump],
                data[idx - (2 * jump)],
                data[idx - (3 * jump)],
            ]
            .eq(XMAS)
                || [
                    data[idx],
                    data[idx - jump],
                    data[idx - (2 * jump)],
                    data[idx - (3 * jump)],
                ]
                .eq(SAMX))
        {
            found += 1;
        }
        // up-left:
        if idx >= (3 * (jump + 1))
            && ([
                data[idx],
                data[idx - (jump + 1)],
                data[idx - (2 * (jump + 1))],
                data[idx - (3 * (jump + 1))],
            ]
            .eq(XMAS)
                || [
                    data[idx],
                    data[idx - (jump + 1)],
                    data[idx - (2 * (jump + 1))],
                    data[idx - (3 * (jump + 1))],
                ]
                .eq(SAMX))
        {
            found += 1;
        }
        // up-right
        if idx >= (3 * (jump - 1))
            && ([
                data[idx],
                data[idx - (jump - 1)],
                data[idx - (2 * (jump - 1))],
                data[idx - (3 * (jump - 1))],
            ]
            .eq(XMAS)
                || [
                    data[idx],
                    data[idx - (jump - 1)],
                    data[idx - (2 * (jump - 1))],
                    data[idx - (3 * (jump - 1))],
                ]
                .eq(SAMX))
        {
            found += 1;
        }
        // down-right:
        if idx + (3 * (jump + 1)) < data.len()
            && ([
                data[idx],
                data[idx + (jump + 1)],
                data[idx + (2 * (jump + 1))],
                data[idx + (3 * (jump + 1))],
            ]
            .eq(XMAS)
                || [
                    data[idx],
                    data[idx + (jump + 1)],
                    data[idx + (2 * (jump + 1))],
                    data[idx + (3 * (jump + 1))],
                ]
                .eq(SAMX))
        {
            found += 1;
        }
        // down-left:
        if idx + (3 * (jump - 1)) < data.len()
            && ([
                data[idx],
                data[idx + (jump - 1)],
                data[idx + (2 * (jump - 1))],
                data[idx + (3 * (jump - 1))],
            ]
            .eq(XMAS)
                || [
                    data[idx],
                    data[idx + (jump - 1)],
                    data[idx + (2 * (jump - 1))],
                    data[idx + (3 * (jump - 1))],
                ]
                .eq(SAMX))
        {
            found += 1;
        }
    }
    found
}

pub fn main() {
    let data = common::read_file::<4>();
    let parsed = parse(&data);
    println!("{}", part_one(&data, parsed));
}
