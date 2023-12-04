#![feature(test)]
extern crate test;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use aoc2023::common;

pub fn parse(input: &[u8]) -> (Vec<&[u8]>, Vec<(usize, usize)>) {
    let lines = input.split(|c| b'\n'.eq(c)).collect::<Vec<_>>();
    let positions = lines
        .iter()
        .zip(0..lines.len())
        .map(|(line, lineno)| {
            line.iter()
                .zip(0..line.len())
                .filter_map(|(c, pos)| {
                    (!c.is_ascii_alphanumeric() && b'.'.ne(c)).then(|| (lineno, pos))
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    (lines, positions)
}

pub fn part_one(lines: &[&[u8]], specials: &[(usize, usize)]) -> usize {
    let mut part_sum = 0;
    lines.iter().zip(0..lines.len()).for_each(|(line, lineno)| {
        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut cur_num = 0;
        for chr in line.iter() {
            let is_digit = chr.is_ascii_digit();
            if is_digit {
                cur_num = (10 * cur_num) + (chr - b'0') as usize;
            }
            if !is_digit || end == line.len() - 1 {
                if cur_num > 0 {
                    let left = match start {
                        0 => 0,
                        n => n - 1,
                    };
                    let top = match lineno {
                        0 => 0,
                        n => n - 1,
                    };
                    'inner: for y in top..=(lineno + 1) {
                        if (left..=end).any(|x| specials.contains(&(y, x))) {
                            part_sum += cur_num;
                            break 'inner;
                        }
                    }
                    start = end;
                }
                start += 1;
                cur_num = 0;
            }
            end += 1;
        }
    });
    part_sum
}

pub fn part_two(lines: &[&[u8]], specials: &[(usize, usize)]) -> usize {
    specials
        .iter()
        .filter_map(|(y, x)| {
            (b'*' == lines[*y][*x]).then(|| {
                let mut power = 0;
                for lineno in (y - 1)..=(y + 1) {
                    let mut start: usize = 0;
                    let mut end: usize = 0;
                    let mut cur_num = 0;
                    if let Some(line) = lines.get(lineno) {
                        for chr in line.iter() {
                            if chr.is_ascii_digit() {
                                cur_num = (10 * cur_num) + (chr - b'0') as usize;
                                end += 1;
                                if end < line.len() {
                                    continue;
                                }
                            }
                            if cur_num > 0 && (start.le(x) && end.ge(x)) {
                                match power {
                                    0 => power = cur_num,
                                    _ => return power * cur_num,
                                }
                            }
                            cur_num = 0;
                            start = end;
                            end += 1;
                        }
                    }
                }
                0
            })
        })
        .sum()
}

pub fn main() {
    let input = common::read_file::<3>();
    let (lines, specials) = parse(&input);
    println!("{}", part_one(&lines, &specials));
    println!("{}", part_two(&lines, &specials));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file::<3>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(parse(test::black_box(&input)), parsed))
    }
    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_file::<3>();
        let (lines, specials) = parse(&input);
        b.iter(|| assert_eq!(part_one(test::black_box(&lines), &specials), 539433))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_file::<3>();
        let (lines, specials) = parse(&input);
        b.iter(|| assert_eq!(part_two(test::black_box(&lines), &specials), 75847567))
    }
}
