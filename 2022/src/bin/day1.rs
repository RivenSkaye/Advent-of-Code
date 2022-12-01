#![feature(test)]
extern crate test;

use aoc2022::common;

pub fn get_data(data: String) -> Vec<usize> {
    data.split("\n\n")
        .map(|inv| {
            inv.lines()
                .map(|item| unsafe { usize::from_str_radix(item, 10).unwrap_unchecked() })
                .sum::<usize>()
        })
        .collect()
}

pub fn part_one(data: &Vec<usize>) -> usize {
    unsafe { *data.iter().max().unwrap_unchecked() }
}

pub fn part_two(mut data: Vec<usize>) -> usize {
    data.sort();
    data.iter().rev().take(3).sum()
}

pub fn main() {
    let data = common::read_file(1);
    let parsed = get_data(data);
    println!("Part one: {}", part_one(&parsed));
    println!("Part two: {}", part_two(parsed));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file(1);
        b.iter(|| get_data(test::black_box(input.clone())))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = get_data(common::read_file(1));
        b.iter(|| test::black_box(part_one(&input)))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = get_data(common::read_file(1));
        b.iter(|| part_two(test::black_box(input.clone())))
    }
}
