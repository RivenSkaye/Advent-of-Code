#![feature(test)]
extern crate test;

use aoc2022::common;

pub fn get_data<'a>(data: &'a String) -> impl Iterator<Item = usize> + 'a {
    data.split("\n\n").map(|inv| {
        inv.lines()
            .map(|item| unsafe { usize::from_str_radix(item, 10).unwrap_unchecked() })
            .sum::<usize>()
    })
}

#[cfg(not(test))]
pub fn part_one<'a>(data: impl Iterator<Item = usize> + 'a) -> usize {
    data.max().unwrap()
}

#[cfg(test)]
pub fn part_one(data: Vec<usize>) -> usize {
    *data.iter().max().unwrap()
}

pub fn part_two(mut data: Vec<usize>) -> usize {
    data.sort();
    data.iter().rev().take(3).sum()
}

pub fn main() {
    let data = common::read_file(1);
    //let parsed = get_data(&data);
    //println!("Part one: {}", part_one(parsed));
    let parsed = get_data(&data);
    println!("Part two: {}", part_two(parsed.collect()));
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
        let input = get_data(&read).collect::<Vec<usize>>();
        b.iter(|| part_one(input.clone()))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let read = common::read_file(1);
        let input = get_data(&read).collect::<Vec<usize>>();
        b.iter(|| part_two(test::black_box(input.clone())))
    }
}
