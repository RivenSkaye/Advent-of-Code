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

fn solve<const TOLERANCE: usize>(data: &Vec<Vec<usize>>) -> usize {
    let margin = TOLERANCE + 1;
    data.iter()
        .filter(|&line| {
            if line[0] == line[1] {
                return false;
            }
            let go_up = line[0] < line[1];
            line.array_windows::<2>()
                .filter(|[a, b]| {
                    if go_up {
                        a < b && a.abs_diff(*b) < 4
                    } else {
                        a > b && a.abs_diff(*b) < 4
                    }
                })
                .count()
                >= line.len() - margin
        })
        .count()
}

pub fn part_one(data: &Vec<Vec<usize>>) -> usize {
    solve::<0>(data)
}

pub fn part_two(data: &Vec<Vec<usize>>) -> usize {
    solve::<1>(data)
}

pub fn main() {
    let data = common::read_file::<2>();
    let parsed = parse(&data);
    println!("{}", part_one(&parsed));
    println!("{}", part_two(&parsed));
}
