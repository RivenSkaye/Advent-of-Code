#![feature(test)]
extern crate test;

use aoc2022::common;

pub fn get_data(data: &str) -> Vec<usize> {
    data.split("\n\n")
        .map(|inv| {
            inv.lines()
                .map(|item| unsafe { item.parse::<usize>().unwrap_unchecked() })
                .sum::<usize>()
        })
        .collect()
}
pub fn part_one(data: &Vec<usize>) -> usize {
    *data.iter().max().unwrap()
}

pub fn part_two(mut data: Vec<usize>) -> usize {
    data.sort();
    data.iter().rev().take(3).sum()
}

pub fn main() {
    let data = common::read_file(1);
    let parsed = get_data(&data);
    part_one(&parsed);
    println!("Part two: {}", part_two(parsed));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file(1);
        b.iter(|| get_data(test::black_box(&input)))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let read = common::read_file(1);
        let input = get_data(&read);
        b.iter(|| assert_eq!(part_one(test::black_box(&input)), 69206))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let read = common::read_file(1);
        let input = get_data(&read);
        b.iter(|| assert_eq!(part_two(test::black_box(input.clone())), 197400))
    }
}
