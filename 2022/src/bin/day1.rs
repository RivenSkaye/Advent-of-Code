use aoc2022::common;

pub fn get_data() -> Vec<usize> {
    common::read_file(1)
        .split("\n\n")
        .map(|inv| {
            inv.lines()
                .map(|item| unsafe { usize::from_str_radix(item, 10).unwrap_unchecked() })
                .sum::<usize>()
        })
        .collect()
}

pub fn part_one(data: &Vec<usize>) -> usize {
    unsafe { *data.iter().max().unwrap_unchecked() }
}

pub fn part_two(mut data: Vec<usize>) -> usize {
    data.sort();
    data.iter().rev().take(3).sum()
}

pub fn main() {
    let data = get_data();
    println!("Part one: {}", part_one(&data));
    println!("Part two: {}", part_two(data));
}
