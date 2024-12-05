#![feature(test)]
extern crate test;

use std::{collections::HashSet, io::BufRead};

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use aoc2024::common;

pub fn parse(input: &[u8]) -> (HashSet<(usize, usize)>, Vec<Vec<usize>>) {
    let mut line_iter = input.lines();
    let failures = line_iter
        .by_ref()
        .map_while(|line| {
            if let Ok(l) = line {
                (!l.is_empty()).then(|| {
                    let (first, last) = l.split_once('|').unwrap();
                    (
                        common::stoi(first.as_bytes()),
                        common::stoi(last.as_bytes()),
                    )
                })
            } else {
                None
            }
        })
        .collect();
    let prints = line_iter
        .map(|line| {
            line.unwrap()
                .split(',')
                .map(|num| common::stoi(num.as_bytes()))
                .collect()
        })
        .collect();
    (failures, prints)
}

#[inline]
fn check_job(failures: &HashSet<(usize, usize)>, job: &[usize]) -> Result<usize, (usize, usize)> {
    let mut printed = Vec::with_capacity(job.len());
    let middle = job.len() / 2; // integer division truncates
    for page in job {
        if let Some(wrong) = failures
            .iter()
            .find(|(f, p)| job.contains(f) && !printed.contains(f) && p == page)
        {
            return Err(*wrong);
        } else {
            printed.push(*page);
        }
    }
    // we know this exists, this should not generate a bounds check
    Ok(job[middle])
}

pub fn part_one(failures: &HashSet<(usize, usize)>, prints: &[Vec<usize>]) -> usize {
    prints
        .iter()
        .filter_map(|job| check_job(failures, job).ok())
        .sum()
}

pub fn part_two(failures: &HashSet<(usize, usize)>, prints: &mut [Vec<usize>]) -> usize {
    prints
        .iter_mut()
        .filter_map(|job| {
            let mut res = None;
            while let Some(off) = check_job(failures, job).err() {
                let a = job.iter().position(|e| off.0.eq(e)).unwrap();
                let b = job.iter().position(|e| off.1.eq(e)).unwrap();
                job.swap(a, b);
                res = Some(job[job.len() / 2]);
            }
            res
        })
        .sum()
}

pub fn main() {
    let input = common::read_file::<5>();
    let (fails, mut prints) = parse(&input);
    println!("{}", part_one(&fails, &prints));
    println!("{}", part_two(&fails, &mut prints));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file::<5>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(parse(test::black_box(&input)), parsed))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_file::<5>();
        let parsed = parse(&input);
        b.iter(|| {
            assert_eq!(
                part_one(
                    test::black_box(&parsed.0),
                    test::black_box(&parsed.1.clone())
                ),
                5108
            )
        })
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_file::<5>();
        let parsed = parse(&input);
        b.iter(|| {
            assert_eq!(
                part_two(
                    test::black_box(&parsed.0),
                    test::black_box(&mut parsed.1.clone())
                ),
                7380
            )
        })
    }

    #[bench]
    fn part2clone_diff(b: &mut test::Bencher) {
        let input = common::read_file::<5>();
        let parsed = parse(&input);
        b.iter(|| {
            assert_eq!(test::black_box(&mut parsed.1.clone()), &parsed.1);
        })
    }
}
