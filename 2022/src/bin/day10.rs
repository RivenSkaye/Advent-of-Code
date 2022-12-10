use aoc2022::common::read_file;

pub fn parse(input: &str) -> Vec<(i64, i64)> {
    let mut x_reg = 1;
    input
        .lines()
        .map(|line| match line.starts_with('a') {
            true => {
                let num: i64 = line.split_once(" ").unwrap().1.parse().unwrap();
                x_reg += num;
                (2, x_reg)
            }
            false => (1, x_reg),
        })
        .collect()
}

pub fn part_one(input: &Vec<(i64, i64)>) -> i64 {
    let mut cycles = 0;
    let mut sigstrength = 0;
    for i in 0..input.len() {
        let next = cycles + input[i].0;
        match (cycles, next) {
            (18 | 19, 20..) => sigstrength += input[i - 1].1 * 20,
            (58 | 59, 60..) => sigstrength += input[i - 1].1 * 60,
            (98 | 99, 100..) => sigstrength += input[i - 1].1 * 100,
            (138 | 139, 140..) => sigstrength += input[i - 1].1 * 140,
            (178 | 179, 180..) => sigstrength += input[i - 1].1 * 180,
            (218 | 219, 220..) => sigstrength += input[i - 1].1 * 220,
            _ => (),
        }
        cycles = next;
    }
    sigstrength
}

pub fn main() {
    let data = read_file::<10>();
    let parsed = parse(&data);
    println!("Part one: {}", part_one(&parsed));
}
