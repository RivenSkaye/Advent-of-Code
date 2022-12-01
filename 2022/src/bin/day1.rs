use aoc2022::common;

pub fn part_one() -> usize {
    common::read_file(1)
        .split("\n\n")
        .map(|inv| {
            inv.lines()
                .map(|item| usize::from_str_radix(item, 10).unwrap_or(0))
                .sum()
        })
        .max()
        .unwrap_or(0)
}

pub fn part_two() -> usize {
    let mut cals = common::read_file(1)
        .split("\n\n")
        .map(|inv| {
            inv.lines()
                .map(|item| usize::from_str_radix(item, 10).unwrap_or(0))
                .sum()
        })
        .collect::<Vec<usize>>();
    cals.sort();
    cals.reverse();
    cals.truncate(3);
    cals.iter().sum()
}

pub fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
