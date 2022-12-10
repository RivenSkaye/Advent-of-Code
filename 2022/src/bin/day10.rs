use aoc2022::common::read_file;

pub fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| match line.starts_with('a') {
            true => (2, line.split_once(" ").unwrap().1.parse().unwrap()),
            false => (1, 0),
        })
        .collect()
}

pub fn part_one<const END: i64>(input: &Vec<(i64, i64)>) -> i64 {
    // start at 20
    let mut cycles = 0;
    let mut x_reg = 1;
    let mut sigstrength = 0;
    let mut last = 0;
    let mut lastval = 0;
    for (instruction, value) in input {
        let comp = if last == 2 && cycles & 1 == 0 {
            cycles
        } else {
            cycles + 1
        };
        match comp {
            20 | 60 | 100 | 140 | 180 => {
                sigstrength += if comp == cycles {
                    x_reg - lastval
                } else {
                    x_reg
                } * (comp);
                println!("{cycles}:  {x_reg} * {} = {}", comp, x_reg * (comp))
            }
            _ => (),
        };
        cycles += *instruction;
        if cycles > END - 2 {
            sigstrength += x_reg * END;
            println!("{cycles}:  {x_reg} * {} = {}", END, x_reg * END);
            break;
        }
        if *instruction == 2 {
            x_reg += *value;
            lastval = *value;
        }
        last = *instruction;
    }
    sigstrength
}

pub fn main() {
    let data = read_file::<10>();
    let parsed = parse(&data);
    println!("Part one: {}", part_one::<220>(&parsed));
}
