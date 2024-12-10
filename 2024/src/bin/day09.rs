#![feature(let_chains, array_chunks, test)]
extern crate test;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use aoc2024::common;

pub fn parse(input: &[u8]) -> (Vec<usize>, usize) {
    // We ignore the first element because it'll have ID 0, and thus never contributes to the value anyway
    let mut diskmap = Vec::with_capacity(input[1..].iter().map(|c| (c - b'0') as usize).sum());
    for ([blank, file], id) in input[1..].array_chunks::<2>().zip(1..) {
        (0..(blank - b'0')).for_each(|_| diskmap.push(0));
        (0..(file - b'0')).for_each(|_| diskmap.push(id));
    }
    (diskmap, (input[0] - b'0') as usize)
}

pub fn part_one(disk: &[usize], offset: usize) -> usize {
    let mut compacted = disk.to_vec();
    let end = compacted.len();
    let mut backiter = (0..end).rev().filter(|&i| disk[i] > 0);
    for idx in 0..compacted.len() {
        if compacted[idx] == 0
            && let Some(back) = backiter.next()
        {
            if idx > back {
                break;
            }
            compacted.swap(idx, back);
        }
    }
    compacted
        .iter()
        .zip(offset..)
        .filter_map(|(a, b)| (0.lt(a) && b > 0).then(|| a * b))
        .sum()
}

pub fn part_two(diskmap: &[usize], offset: usize) -> usize {
    let mut lengths = Vec::with_capacity(diskmap.len() / 2);
    let mut file_lengths = vec![0; diskmap.iter().max().unwrap() + 1];
    let mut start = 0;
    let mut len = 0;
    // Create a map of all the blocks
    for block in diskmap {
        if 0.lt(block) {
            file_lengths[*block] += 1;
            if len > 0 {
                lengths.push((start, len));
                start += len;
                len = 0;
            }
            start += 1;
            continue;
        }
        len += 1;
    }
    let mut ordered = diskmap.to_vec();
    let mut idx = diskmap.len() - 1;
    for (id, file) in file_lengths.iter().enumerate().rev() {
        if let Some((empty_idx, mut first_empty, mut empty_len)) = lengths
            .iter()
            .enumerate()
            .find_map(|(e, (s, l))| (file.le(l) && id > e).then_some((e, *s, *l)))
        {
            for i in 0..*file {
                ordered.swap(idx - i, first_empty);
                first_empty += 1;
                empty_len -= 1;
            }
            if empty_len > 0 {
                lengths[empty_idx] = (first_empty, empty_len)
            } else {
                lengths.remove(empty_idx);
            }
        }
        if id == 1 {
            break;
        }
        idx -= file;
        while ordered[idx] == 0 || ordered[idx] > id {
            idx -= 1;
        }
    }
    ordered
        .iter()
        .zip(offset..)
        .filter_map(|(a, b)| (0.lt(a) && b > 0).then(|| a * b))
        .sum()
}

pub fn main() {
    let input = common::read_file::<9>();
    let (parsed, offset) = parse(&input);
    println!("{}", part_one(&parsed, offset));
    println!("{}", part_two(&parsed, offset));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file::<9>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(parse(test::black_box(&input)), parsed))
    }
    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_file::<9>();
        let parsed = parse(&input);
        b.iter(|| {
            assert_eq!(
                part_one(test::black_box(&parsed.0), test::black_box(parsed.1)),
                6241633730082
            )
        })
    }
    /*
    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_file::<8>();
        let parsed = Grid::from(input.as_slice());
        b.iter(|| assert_eq!(part_two(test::black_box(&parsed)), 809))
    }*/
}
