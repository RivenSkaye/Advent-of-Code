#![feature(pattern)]
pub fn get_data<'a>(data: &'a str) -> impl Iterator<Item = i64> + 'a {
    data.split("\n\n").map(|inv| {
        inv.lines()
            .map(|item| unsafe { i64::from_str_radix(item, 10).unwrap_unchecked() })
            .sum::<i64>()
    })
}

pub fn part_one<'a>(data: impl Iterator<Item = i64> + 'a) -> i64 {
    data.max().unwrap()
}

pub fn run(data: &str) -> i64 {
    let parsed = get_data(data);
    part_one(parsed)
    //println!("Part two: {}", part_two(parsed));
}
