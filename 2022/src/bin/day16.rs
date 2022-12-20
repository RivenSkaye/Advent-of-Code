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
        .map(|sline| {
            let line = sline.as_bytes();
            // If not for the different text when a valve only has a single tunnel,
            // this wouldn't be necessary. But the different text does three things:
            // 1) removes an s from tunnels (net -1)
            // 2) adds an s to lead (net 0)
            // 3) removes an s from valves (net -1)
            // This means that the tunnel list can start at positions:
            // 50 => 2 digit flow value, multiple tunnels. 2 chars back is always an `s`
            // 49 => 1 digit flow value, multiple tunnels. 1 char back is always a space
            // 49 => 2 digit flow value, one tunnel. 1 char back is always a space
            // 48 => 1 digit flow value, one tunnel. This char is a capital A-Z
            let tunnels = match line[48] {
                b's' => 50,
                b' ' => 49,
                _ => 48,
            };
            (
                line[6..8].iter().fold(0, |a, b| (a << 8) | *b as u16),
                Valve {
                    rate: line[23..25].iter().fold(0, |a, b| match b {
                        b';' => a,
                        _ => (10 * a) + (b - b'0') as i64,
                    }),
                    connections: line[tunnels..]
                        .split(|byte| b','.eq(byte) || b' '.eq(byte))
                        .filter(|slc| slc.len() > 0)
                        .map(|valve| valve.iter().fold(0, |a, b| (a << 8) | *b as u16))
                        .collect(),
                    open: false,
                },
            )
        })
        .collect()
}

pub fn main() {
    let input = read_file::<16>();
    let _parsed = parse(&input);
}
