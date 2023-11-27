use aoc2015::common;
use md5::{Md5, Digest};
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const INPUT: &'static str = "yzbqklnj";

fn part_one() -> usize {
    for i in 0..=usize::MAX {
        let dig = format!("{:x}", Md5::digest(format!("{INPUT}{i}")));
        if dig.starts_with("00000") { return i }
    }
    unreachable!()
}

fn part_two(start: usize) -> usize {
    for i in start..=usize::MAX {
        let dig = format!("{:x}", Md5::digest(format!("{INPUT}{i}")));
        if dig.starts_with("000000") { return i }
    }
    unreachable!()
}

fn main() {
    let p1 = part_one();
    println!("{}", p1);
    println!("{}", part_two(p1))
}