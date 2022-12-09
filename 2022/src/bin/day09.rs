use aoc2022::common::read_file;
use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<(u8, u16)> {
    input
        .lines()
        .map(|line| {
            let parts = unsafe { line.split_once(" ").unwrap_unchecked() };
            match parts {
                ("L", x) => (b'L', unsafe { x.parse::<u16>().unwrap_unchecked() }),
                ("R", y) => (b'R', unsafe { y.parse::<u16>().unwrap_unchecked() }),
                ("D", y) => (b'D', unsafe { y.parse::<u16>().unwrap_unchecked() }),
                (_, x) => (b'U', unsafe { x.parse::<u16>().unwrap_unchecked() }),
            }
        })
        .collect()
}

#[inline]
fn walk<const LENGTH: usize>(parsed: Vec<(u8, u16)>) -> i64 {
    let mut coords = [(0, 0); LENGTH];
    parsed
        .iter()
        .map(|(dir, len)| {
            (0..*len).into_iter().for_each(|step| match dir {
                b'L' => coords[0].1 -= 1,
                b'R' => coords[0].1 += 1,
                b'D' => coords[0].0 -= 1,
                _ => coords[0].0 += 1,
            });
            // code go brrr
            (*dir as isize, *len)
        })
        .collect::<HashSet<(isize, u16)>>()
        .len() as i64
}

pub fn part_one(parsed: Vec<(u8, u16)>) -> i64 {
    walk::<2>(parsed)
}

pub fn part_two(parsed: Vec<(u8, u16)>) -> i64 {
    walk::<10>(parsed)
}

pub fn main() {
    let data = read_file::<9>();
    let parsed = parse(&data);
}
