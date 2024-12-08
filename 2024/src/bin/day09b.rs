#![feature(array_chunks, test)]

use aoc2024::common;

pub fn parse(input: &[u8]) -> (Vec<usize>, usize) {
    println!("{}", input.len());
    // We ignore the first element because it'll have ID 0, and thus never contributes to the value anyway
    let mut diskmap = Vec::with_capacity(input[1..].iter().map(|c| (c - b'0') as usize).sum());
    for ([blank, file], id) in input[1..].array_chunks::<2>().zip(1..) {
        (0..(blank - b'0')).for_each(|_| diskmap.push(0));
        (0..(file - b'0')).for_each(|_| diskmap.push(id));
    }
    (diskmap, (input[0] - b'0') as usize)
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
    for (file, id) in file_lengths.iter().zip(0..file_lengths.len()).rev() {
        println!("Moving {file} `{id}` blocks");
        if let Some((empty_idx, mut first_empty, mut empty_len)) = lengths
            .iter()
            .enumerate()
            .find_map(|(e, (s, l))| file.le(l).then_some((e, *s, *l)))
        {
            println!(
                "Moving {file} blocks into {} through {}",
                first_empty + offset,
                first_empty + empty_len + offset
            );
            for i in 0..*file {
                ordered.swap(idx - i, first_empty);
                first_empty += 1;
                empty_len -= 1;
            }
            lengths[empty_idx] = (first_empty, empty_len)
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
        .filter_map(|(a, b)| {
            (0.lt(a) && b > 0).then(|| {
                println!("{a} * {b} = {}", a * b);
                a * b
            })
        })
        .sum()
}

pub fn main() {
    let data = common::read_file::<9>();
    let (parsed, offset) = parse(&data);
    println!("{}", part_two(&parsed, offset));
    // I'm getting a resultant checksum of 9514159291881. Which is apparently too high.
}
