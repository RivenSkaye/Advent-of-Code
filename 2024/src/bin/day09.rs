#![feature(let_chains, array_chunks, test)]
extern crate test;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use aoc2024::common;

pub fn parse(input: &[u8]) -> Vec<Option<usize>> {
    let mut base = Vec::with_capacity(input.len());
    let mut ids = 0..;
    for ([file, empty], id) in input.array_chunks::<2>().zip(ids.by_ref()) {
        (0..((file - b'0') as usize)).for_each(|_| base.push(Some(id)));
        (0..((empty - b'0') as usize)).for_each(|_| base.push(None));
    }
    let next = ids.next();
    (0..((input[input.len() - 1] - b'0') as usize)).for_each(|_| base.push(next));
    base
}

pub fn part_one(disk: &[Option<usize>]) -> usize {
    let mut compacted = disk.to_vec();
    let end = compacted.len();
    let mut backiter = (0..end).rev().filter(|&i| disk[i].is_some());
    for idx in 0..compacted.len() {
        if compacted[idx].is_none()
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
        .filter_map(|a| *a)
        .enumerate()
        .fold(0, |last, (pos, fid)| last + (pos * fid))
}

pub fn main() {
    let input = common::read_file::<9>();
    let parsed = parse(&input);
    println!("{}", part_one(&parsed))
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
        b.iter(|| assert_eq!(part_one(test::black_box(&parsed)), 6241633730082))
    }
    /*
    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_file::<8>();
        let parsed = Grid::from(input.as_slice());
        b.iter(|| assert_eq!(part_two(test::black_box(&parsed)), 809))
    }*/
}
