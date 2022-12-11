#![feature(test)]
use aoc2022::common::read_file;

pub fn parse(input: &str) -> Vec<(i64, i64)> {
    let mut x_reg = 1;
    input
        .lines()
        .map(|line| match line.starts_with('a') {
            true => {
                let num: i64 =
                    unsafe { line.split_once(" ").unwrap().1.parse().unwrap_unchecked() };
                x_reg += num;
                (2, x_reg)
            }
            false => (1, x_reg),
        })
        .collect()
}

pub fn part_one(input: &Vec<(i64, i64)>) -> i64 {
    let mut cycles = 0;
    let mut sigstrength = 0;
    for i in 0..input.len() {
        let next = cycles + input[i].0;
        match (cycles, next) {
            (18 | 19, 20..) => sigstrength += input[i - 1].1 * 20,
            (58 | 59, 60..) => sigstrength += input[i - 1].1 * 60,
            (98 | 99, 100..) => sigstrength += input[i - 1].1 * 100,
            (138 | 139, 140..) => sigstrength += input[i - 1].1 * 140,
            (178 | 179, 180..) => sigstrength += input[i - 1].1 * 180,
            (218 | 219, 220..) => sigstrength += input[i - 1].1 * 220,
            _ => (),
        }
        cycles = next;
    }
    sigstrength
}

pub fn part_two(input: &Vec<(i64, i64)>) -> String {
    let mut cycles = 0;
    let mut x_reg = 0_i64;
    let mut output = String::with_capacity(240);
    input.iter().for_each(|(steps, res)| {
        for i in cycles..cycles + steps {
            output.push(if x_reg.abs_diff(if i < 40 { i } else { 0 }) < 2 {
                '#'
            } else {
                '.'
            });
        }
        x_reg = *res;
        cycles = (cycles + steps) % 40;
    });
    output
}

pub fn main() {
    let data = read_file::<10>();
    let parsed = parse(&data);
    println!("Part one: {}", part_one(&parsed));
    let mut i = 1;
    println!("");
    for c in part_two(&parsed).chars() {
        print!("{c}");
        if i == 0 {
            println!("");
        }
        i = (i + 1) % 40;
    }
}

#[cfg(test)]
mod aoc_benching {
    extern crate test;
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = read_file::<10>();
        b.iter(|| parse(test::black_box(&input)))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let read = read_file::<10>();
        let input = parse(&read);
        b.iter(|| assert_eq!(part_one(test::black_box(&input)), 13440))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let read = read_file::<10>();
        let input = parse(&read);
        b.iter(|| assert_eq!(part_two(test::black_box(&input)), "###..###..####..##..###...##..####..##..#..#.#..#....#.#..#.#..#.#..#....#.#..#.#..#.###....#..#....#..#.#..#...#..#..#.###..#..#..#...#.##.###..####..#...####.#....#..#.#....#..#.#.#..#..#.#....#..#.#....###..####..###.#..#.#..#.####.#..#."))
    }
}
