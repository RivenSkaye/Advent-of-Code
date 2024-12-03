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

pub fn p2_safe(data: &Vec<Vec<usize>>) -> usize {
    let mut safe = data.len();
    fn check_line(line: &Vec<usize>) -> usize {
        // bounds check once - saves 2ms!
        assert!(line.len() > 3);
        // gracefully stolen from kageru
        let go_up = [line[0] < line[1], line[1] < line[2], line[2] < line[3]]
            .into_iter()
            .filter(|&b| b)
            .count()
            > 1;
        if let Some(idx) = line
            .array_windows::<2>()
            .zip(1_usize..)
            .find(|&([a, b], _)| {
                if go_up {
                    !(a < b && a.abs_diff(*b) < 4)
                } else {
                    !(a > b && a.abs_diff(*b) < 4)
                }
            })
            .map(|(_, idx)| idx)
        {
            return idx;
        }
        0
    }
    for line in data {
        let fail = check_line(line);
        if fail > 0 {
            let mut not_next = line.clone();
            not_next.remove(fail);
            if check_line(&not_next) > 0 {
                let mut not_cur = line.clone();
                not_cur.remove(fail - 1);
                if check_line(&not_cur) > 0 {
                    unsafe { safe = safe.unchecked_sub(1) }
                }
            }
        }
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
    println!("{}", p2_safe(&parsed));
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

    #[bench]
    fn part2safebench(b: &mut test::Bencher) {
        let input = common::read_file::<2>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(p2_safe(test::black_box(&parsed)), 566))
    }
}
