#![feature(array_windows, test)]
extern crate test;
use aoc2024::common;

#[inline(always)]
fn stoi_split(s: &[u8]) -> (usize, usize) {
    let mut s_iter = s.iter();
    (
        s_iter
            .by_ref()
            .take_while(|c| b','.ne(c))
            .fold(0, |i, c| (10 * i) + (c - b'0') as usize),
        s_iter.fold(0, |i, c| (10 * i) + (c - b'0') as usize),
    )
}

fn part_one(data: &[u8]) -> usize {
    // minimum length of a valid sequence is mul(x,y) (8)
    if data.len() < 8 {
        return 0;
    }
    let mut total = 0;
    let mut idx = 0;
    'outer: loop {
        if idx > data.len() - 8 {
            break;
        }
        if data[idx..(idx + 4)].ne(b"mul(") {
            //println!("{:?}", String::from_utf8_lossy(&data[idx..(idx + 4)]));
            idx += 1;
            continue;
        }
        idx += 4;
        let start = idx;
        let mut comma = false;
        while idx < data.len() && data[idx] != b')' {
            if data[idx] == b',' {
                if comma {
                    continue 'outer;
                }
                comma = true;
            }
            if !b"1234567890,".contains(&data[idx]) {
                idx += 1;
                continue 'outer;
            }
            idx += 1;
        }
        if !comma {
            continue;
        }
        // The text clearly states that all numerics are one to three digits, plus the comma that's 7 max
        if idx - start > 8 {
            continue;
        }
        let (a, b) = stoi_split(&data[start..idx]);
        total += a * b;
        idx += 1;
    }
    total
}

pub fn part_two(data: &[u8]) -> usize {
    // minimum length of a valid sequence is mul(x,y) (8)
    if data.len() < 8 {
        return 0;
    }
    let mut total = 0;
    let mut idx = 0;
    let mut do_mul = true;
    'outer: loop {
        if idx > data.len() - 8 {
            break;
        }
        if data[idx..(idx + 4)].eq(b"do()") {
            do_mul = true;
            idx += 4;
            continue;
        }
        if data[idx..(idx + 7)].eq(b"don't()") {
            do_mul = false;
            idx += 7;
            continue;
        }
        if !do_mul {
            idx += 1;
            continue;
        }
        if data[idx..(idx + 4)].ne(b"mul(") {
            //println!("{:?}", String::from_utf8_lossy(&data[idx..(idx + 4)]));
            idx += 1;
            continue;
        }
        idx += 4;
        let start = idx;
        let mut comma = false;
        while idx < data.len() && data[idx] != b')' {
            if data[idx] == b',' {
                if comma {
                    continue 'outer;
                }
                comma = true;
            }
            if !b"1234567890,".contains(&data[idx]) {
                idx += 1;
                continue 'outer;
            }
            idx += 1;
        }
        if !comma {
            continue;
        }
        // The text clearly states that all numerics are one to three digits, plus the comma that's 7 max
        if idx - start > 7 {
            continue;
        }
        let (a, b) = stoi_split(&data[start..idx]);
        total += a * b;
        idx += 1;
    }
    total
}

pub fn main() {
    let data = common::read_file::<3>();
    println!("{}", part_one(&data));
    println!("{}", part_two(&data));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_file::<3>();
        b.iter(|| assert_eq!(part_one(test::black_box(&input)), 173731097))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_file::<3>();
        b.iter(|| assert_eq!(part_two(test::black_box(&input)), 93729253))
    }
}
