#![feature(array_windows)]

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
    data.iter()
        .filter(|line| {
            let go_up = line[0] < line[1];
            line[1..]
                .array_windows::<2>()
                .filter(|[a, b]| {
                    if go_up {
                        a < b && a.abs_diff(*b) < 4
                    } else {
                        a > b && a.abs_diff(*b) < 4
                    }
                })
                .count()
                > 0
        })
        .count()
}

pub fn part_two(left: &Vec<usize>, right: &Vec<usize>) -> usize {
    0
}

pub fn main() {
    let data = common::read_file::<2>();
    let parsed = parse(&data);
    println!("{}", part_one(&parsed));
}
