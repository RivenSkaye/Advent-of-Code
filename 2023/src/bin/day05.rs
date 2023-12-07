#![feature(array_chunks, array_windows)]
#![feature(test)]
extern crate test;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use aoc2023::common;

pub fn parse(input: &str) -> (Vec<usize>, Vec<Vec<(usize, usize, usize)>>) {
    let mut chunks = input.split("\n\n");
    let seeds = chunks
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|num| common::stoi(num))
        .collect::<Vec<_>>();

    let maps = chunks
        .map(|chunk| {
            chunk
                .lines()
                .skip(1)
                .map(|line| {
                    let mut nums = line.split_ascii_whitespace();
                    let dst_start = common::stoi(nums.next().unwrap());
                    let src_start = common::stoi(nums.next().unwrap());
                    let len = common::stoi(nums.next().unwrap());
                    (src_start, dst_start, len)
                })
                .collect()
        })
        .collect();
    (seeds, maps)
}

pub fn part_one(seeds: &[usize], maps: &[Vec<(usize, usize, usize)>]) -> usize {
    seeds
        .iter()
        .map(|&seed| {
            maps.iter().fold(seed, |last, map| {
                map.iter()
                    .find_map(|&(src, dst, len)| {
                        (src..src + len).contains(&last).then_some(
                            match (dst > src, src.abs_diff(dst)) {
                                (true, n) => last + n,
                                (false, n) => last - n,
                            },
                        )
                    })
                    .unwrap_or(last)
            })
        })
        .min()
        .unwrap()
}

pub fn part_two(seeds: &[usize], maps: &[Vec<(usize, usize, usize)>]) -> usize {
    let seedranges = seeds
        .array_chunks::<2>()
        .map(|&[a, b]| a..a + b)
        .collect::<Vec<_>>();
    let mut locations = maps.last().unwrap().to_owned();
    locations.push((0, 0, 0));
    locations.sort_by_key(|loc| loc.1);
    let loc_ranges = locations
        .array_windows::<2>()
        .map(|&[last, cur]| {
            [
                (last.1.min(cur.1)..last.1.max(cur.1), 0, false),
                (cur.1..cur.1 + cur.2, cur.1.abs_diff(cur.0), cur.1 > cur.0),
            ]
        })
        .flatten()
        .collect::<Vec<_>>();

    loc_ranges
        .iter()
        .find_map(|map| {
            let mut range = map.0.to_owned();
            range.find_map(|loc| {
                let init = match map.2 {
                    false => loc + map.1,
                    true => loc - map.1,
                };
                let seed = maps.iter().rev().skip(1).fold(init, |last, map| {
                    map.iter()
                        .find_map(|&(dst, src, len)| {
                            (src..src + len).contains(&last).then_some(
                                match (src > dst, src.abs_diff(dst)) {
                                    (true, n) => last - n,
                                    (false, n) => last + n,
                                },
                            )
                        })
                        .unwrap_or(last)
                });
                seedranges
                    .iter()
                    .any(|range| range.contains(&seed))
                    .then_some(loc)
            })
        })
        .unwrap()
}

pub fn main() {
    let input = common::read_str::<5>();
    let (seeds, maps) = parse(&input);
    println!("{}", part_one(&seeds, &maps));
    println!("{}", part_two(&seeds, &maps));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_str::<5>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(&parse(test::black_box(&input)), &parsed))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_str::<5>();
        let (seeds, maps) = parse(&input);
        b.iter(|| assert_eq!(part_one(&seeds, test::black_box(&maps)), 26273516))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_str::<5>();
        let (seeds, maps) = parse(&input);
        b.iter(|| assert_eq!(part_two(&seeds, test::black_box(&maps)), 34039469))
    }
}
