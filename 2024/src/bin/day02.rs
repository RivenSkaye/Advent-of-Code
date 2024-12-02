#![feature(array_windows, test)]
extern crate test;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use aoc2024::common;

pub fn parse(input: &[u8]) -> Vec<Vec<usize>> {
    input
        .split(|chr| b'\n'.eq(chr))
        .map(|line| {
            line.split(|chr| chr.is_ascii_whitespace())
                .map(common::stoi)
                .collect()
        })
        .collect()
}

pub fn part_one(data: &Vec<Vec<usize>>) -> usize {
    let mut safe = 0;
    for line in data {
        if line[0] == line[1] {
            continue;
        }
        let go_up = line[0] < line[1];
        if line
            .array_windows::<2>()
            .find(|[a, b]| {
                if go_up {
                    !(a < b && a.abs_diff(*b) < 4)
                } else {
                    !(a > b && a.abs_diff(*b) < 4)
                }
            })
            .is_some()
        {
            continue;
        }
        safe += 1;
    }
    safe
}

pub fn part_two(data: &Vec<Vec<usize>>) -> usize {
    let mut safe = 0;
    for line in data {
        let mut margin = 2;
        let go_up = if line[0] == line[1] {
            if line[1] == line[2] {
                continue;
            }
            margin -= 1;
            line[1] < line[2]
        } else {
            line[0] < line[1]
        };
        if line
            .array_windows::<2>()
            .find(|[a, b]| {
                if (go_up && (a < b && a.abs_diff(*b) < 4))
                    || (!go_up && (a > b && a.abs_diff(*b) < 4))
                {
                    return false;
                }
                margin -= 1;
                margin == 0
            })
            .is_some()
        {
            continue;
        }
        safe += 1;
    }
    safe
}

pub fn main() {
    let data = common::read_file::<2>();
    let parsed = parse(&data);
    println!("{}", part_one(&parsed));
    println!("{}", part_two(&parsed));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file::<2>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(parse(test::black_box(&input)), parsed))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_file::<2>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part_one(test::black_box(&parsed)), 526))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_file::<2>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part_two(test::black_box(&parsed)), 566))
    }
}
