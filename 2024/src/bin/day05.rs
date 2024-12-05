#![feature(array_windows, test)]
extern crate test;

use std::{collections::HashMap, io::BufRead};

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use aoc2024::common;

pub fn parse(input: &[u8]) -> (HashMap<usize, usize>, Vec<Vec<usize>>) {
    let mut line_iter = input.lines();
    let failures = line_iter
        .by_ref()
        .map_while(|line| {
            if let Ok(l) = line {
                (!l.is_empty()).then(|| {
                    let (first, last) = l.split_once('|').unwrap();
                    println!("{first}|{last}");
                    (
                        common::stoi(last.as_bytes()),
                        common::stoi(first.as_bytes()),
                    )
                })
            } else {
                None
            }
        })
        .collect();
    let prints = line_iter
        .map(|line| {
            line.unwrap()
                .split(',')
                .map(|num| common::stoi(num.as_bytes()))
                .collect()
        })
        .collect();
    (failures, prints)
}

pub fn part_one(failures: &HashMap<usize, usize>, prints: &[Vec<usize>]) -> usize {
    prints
        .iter()
        .filter_map(|job| {
            println!("{job:?}");
            let mut printed = Vec::with_capacity(job.len());
            let middle = (job.len() / 2) + 1; // integer division truncates
            for page in job {
                if let Some(fnum) = failures.get(page) {
                    if !printed.contains(fnum) && job.contains(fnum) {
                        println!("{page} was out of order");
                        return None;
                    }
                    printed.push(*page);
                } else {
                    printed.push(*page);
                }
            }
            println!("{} is A-OK", job[middle]);
            // we know this exists, this should not generate a bounds check
            Some(printed[middle])
        })
        .sum()
}

pub fn main() {
    let input = common::read_file::<5>();
    let (fails, prints) = parse(&input);
    println!("{}", part_one(&fails, &prints));
}
