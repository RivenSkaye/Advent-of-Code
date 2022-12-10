#![feature(generic_const_exprs)]
use aoc2022::common::read_file;
use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<(u8, u16)> {
    input
        .lines()
        .map(|line| {
            let parts = unsafe { line.split_once(" ").unwrap_unchecked() };
            match parts {
                ("L", x) => (b'L', x.parse::<u16>().unwrap()),
                ("R", y) => (b'R', y.parse::<u16>().unwrap()),
                ("D", y) => (b'D', y.parse::<u16>().unwrap()),
                (_, x) => (b'U', x.parse::<u16>().unwrap()),
            }
        })
        .collect()
}

fn walk<const TAILS: usize>(parsed: Vec<(u8, u16)>) -> i64 {
    let mut tails = [(0_isize, 0_isize); TAILS];
    let mut coords = HashSet::new();
    coords.insert((0, 0));
    parsed.iter().for_each(|(dir, len)| {
        (0..*len).into_iter().for_each(|_| {
            match *dir {
                b'L' => tails[0].1 -= 1,
                b'R' => tails[0].1 += 1,
                b'D' => tails[0].0 -= 1,
                _ => tails[0].0 += 1,
            }
            (0..TAILS - 1).into_iter().for_each(|cur| {
                let next = cur + 1;
                let vdiff = tails[cur].0 - tails[next].0;
                let hdiff = tails[cur].1 - tails[next].1;
                match (hdiff.abs() > 1, vdiff.abs() > 1) {
                    (true, true) => {
                        tails[next] = (
                            tails[next].0 + vdiff.signum(),
                            tails[next].1 + hdiff.signum(),
                        );
                    }
                    _ => (),
                }
            });
            coords.insert(tails[TAILS - 1]);
        });
    });
    coords.len() as i64
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
    //println!("Part one: {}", part_one(parsed.clone()));
    println!("Part two: {}", part_two(parsed));
}
