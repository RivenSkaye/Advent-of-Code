use std::collections::HashMap;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use aoc2024::common;

pub fn parse(input: &[u8]) -> (Vec<usize>, Vec<usize>) {
    let mut left = Vec::<_>::new();
    let mut right = Vec::<_>::new();
    let mut go_left = true;
    for word in input.split(|chr| chr.is_ascii_whitespace()) {
        if word.len() == 0 {
            continue;
        }
        let numval = common::stoi(word);
        if go_left {
            left.push(numval);
        } else {
            right.push(numval);
        }
        go_left = !go_left;
    }
    (left, right)
}

pub fn part_one(left: &Vec<usize>, right: &Vec<usize>) -> usize {
    let mut l = left.clone();
    l.sort();
    let mut r = right.clone();
    r.sort();
    l.iter()
        .zip(r.iter())
        .map(|(&ln, &rn)| ln.abs_diff(rn) as usize)
        .sum()
}

pub fn part_two(left: &Vec<usize>, right: &Vec<usize>) -> usize {
    let mut known = HashMap::with_capacity(left.len());
    left.iter()
        .map(|num| {
            if let Some(val) = known.get(num) {
                *val
            } else {
                let val = right.iter().filter(|n| num.eq(n)).count() * num;
                known.insert(num, val);
                val
            }
        })
        .sum()
}

pub fn main() {
    let data = common::read_file::<1>();
    let (lvec, rvec) = parse(&data);
    println!("{}", part_one(&lvec, &rvec));
    println!("{}", part_two(&lvec, &rvec));
}
