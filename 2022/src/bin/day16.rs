use aoc2022::common::read_file;
use std::collections::HashMap;

pub struct Valve {
    rate: i64,
    connections: Vec<u16>,
    open: bool,
}

pub fn parse(input: &str) -> HashMap<u16, Valve> {
    input
        .lines()
        .map(|line| {
            (
                line[6..8]
                    .as_bytes()
                    .iter()
                    .fold(0, |a, b| (a << 8) | *b as u16),
                Valve {
                    rate: line[23..25].as_bytes().iter().fold(0, |a, b| match b {
                        b';' => a,
                        _ => (10 * a) + (b - b'0') as i64,
                    }),
                    connections: line[49..]
                        .split(", ")
                        .map(|valve| {
                            valve.as_bytes().iter().fold(0, |a, b| match b {
                                b' ' => a,
                                _ => (a << 8) | *b as u16,
                            })
                        })
                        .collect(),
                    open: false,
                },
            )
        })
        .collect()
}

pub fn main() {
    let input = read_file::<16>();
    let parsed = parse(&input);
}
