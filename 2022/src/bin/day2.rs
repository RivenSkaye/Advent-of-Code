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
        .map(|round| match [round[0], round[2]] {
            [b'A', b'X'] => 3,
            [b'A', b'Y'] => 4,
            [b'A', b'Z'] => 8,
            [b'B', b'X'] => 1,
            [b'B', b'Y'] => 5,
            [b'B', b'Z'] => 9,
            [b'C', b'X'] => 2,
            [b'C', b'Y'] => 6,
            [b'C', b'Z'] => 7,
            _ => unreachable!(),
        })
        .sum()
}

pub fn main() {
    let input = common::read_file(2);
    println!("{}", part_one(get_data(&input)));
    println!("{}", part_two(get_data(&input)));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file(2);
        b.iter(|| get_data(test::black_box(&input)))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let read = common::read_file(2);
        let input = get_data(&read);
        b.iter(|| assert_eq!(part_one(test::black_box(input)), 13565))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let read = common::read_file(2);
        let input = get_data(&read);
        b.iter(|| assert_eq!(part_two(test::black_box(input)), 12424))
    }
}
