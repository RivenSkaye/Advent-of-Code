#![feature(generic_const_exprs)]
#![feature(test)]

use aoc2022::common::read_file;
use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<(u8, u16)> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_once(" ").unwrap();
            match parts {
                ("L", x) => (b'L', unsafe { x.parse::<u16>().unwrap_unchecked() }),
                ("R", y) => (b'R', unsafe { y.parse::<u16>().unwrap_unchecked() }),
                ("D", y) => (b'D', unsafe { y.parse::<u16>().unwrap_unchecked() }),
                ("U", x) => (b'U', unsafe { x.parse::<u16>().unwrap_unchecked() }),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn walk<const TAILS: usize>(parsed: &Vec<(u8, u16)>) -> i64
where
    // We only care about the tails, so why should anyone consuming this API
    // ever worry about +1 for the head? I also think it looks nicer than doing
    // CONSTVAL - 1 repeatedly.
    [(); TAILS + 1]: Sized,
{
    let mut tails = [(0_isize, 0_isize); TAILS + 1];
    let mut coords = HashSet::new();
    parsed.iter().for_each(|(dir, len)| {
        (0..*len).into_iter().for_each(|_| {
            match *dir {
                b'L' => tails[0].1 -= 1,
                b'R' => tails[0].1 += 1,
                b'D' => tails[0].0 -= 1,
                b'U' => tails[0].0 += 1,
                _ => unreachable!(),
            }
            for cur in 0..TAILS {
                let vdiff = tails[cur].0 - tails[cur + 1].0;
                let hdiff = tails[cur].1 - tails[cur + 1].1;
                match vdiff.abs() > 1 || hdiff.abs() > 1 {
                    true => {
                        tails[cur + 1] = (
                            tails[cur + 1].0 + vdiff.signum(),
                            tails[cur + 1].1 + hdiff.signum(),
                        )
                    }
                    _ => continue,
                }
            }
            coords.insert(tails[TAILS]);
        });
    });
    coords.len() as i64
}

pub fn part_one(parsed: &Vec<(u8, u16)>) -> i64 {
    walk::<1>(&parsed)
}

pub fn part_two(parsed: &Vec<(u8, u16)>) -> i64 {
    walk::<9>(&parsed)
}

pub fn main() {
    let data = read_file::<9>();
    let parsed = parse(&data);
    println!("Part one: {}", part_one(&parsed));
    println!("Part two: {}", part_two(&parsed));
}

#[cfg(test)]
mod aoc_benching {
    extern crate test;
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = read_file::<9>();
        b.iter(|| parse(test::black_box(&input)))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let read = read_file::<9>();
        let input = parse(&read);
        b.iter(|| assert_eq!(part_one(test::black_box(&input)), 6236))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let read = read_file::<9>();
        let input = parse(&read);
        b.iter(|| assert_eq!(part_two(test::black_box(&input)), 2449))
    }
}
