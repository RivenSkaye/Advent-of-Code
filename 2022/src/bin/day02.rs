#![feature(test)]
#![feature(array_chunks)]
extern crate test;

use aoc2022::common;

pub fn get_data(data: &str) -> &[u8] {
    data.as_bytes()
}

pub fn part_one(input: &[u8]) -> usize {
    input
        .array_chunks::<4>()
        .map(|round| {
            let other = (round[0] - b'A') + 1;
            let own = round[2] - b'W';
            usize::from(if other == own {
                3 + own
            } else if other < own {
                if own == 3 && other == 1 {
                    own
                } else {
                    own + 6
                }
            } else {
                if own == 1 && other == 3 {
                    7
                } else {
                    own
                }
            })
        })
        .sum()
}

pub fn part_two(input: &[u8]) -> usize {
    input
        .array_chunks::<4>()
        .map(|round| {
            let other = round[0] - b'A';
            let res = round[2] - b'W';
            usize::from(if res == 3 {
                if other == 2 {
                    7
                } else {
                    8 + other
                }
            } else if res == 2 {
                other + 4
            } else if other == 0 {
                3
            } else {
                other
            })
        })
        .sum()
}

pub fn main() {
    let input = common::read_file::<2>();
    println!("{}", part_one(get_data(&input)));
    println!("{}", part_two(get_data(&input)));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file::<2>();
        b.iter(|| get_data(test::black_box(&input)))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let read = common::read_file::<2>();
        let input = get_data(&read);
        b.iter(|| assert_eq!(part_one(test::black_box(input)), 13565))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let read = common::read_file::<2>();
        let input = get_data(&read);
        b.iter(|| assert_eq!(part_two(test::black_box(input)), 12424))
    }
}
