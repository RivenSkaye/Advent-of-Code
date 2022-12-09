use aoc2022::common::read_file;
use std::collections::BTreeSet;

pub fn parse(input: &str) -> Vec<(isize, isize)> {
    input
        .lines()
        .map(|line| {
            let parts = unsafe { line.split_once(" ").unwrap_unchecked() };
            match parts {
                ("D", y) => (-(unsafe { y.parse::<isize>().unwrap_unchecked() }), 0),
                ("U", y) => (unsafe { y.parse::<isize>().unwrap_unchecked() }, 0),
                ("L", x) => (0, -(unsafe { x.parse::<isize>().unwrap_unchecked() })),
                ("R", x) => (0, unsafe { x.parse::<isize>().unwrap_unchecked() }),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn walk(
    change: &(isize, isize),
    t: &mut (isize, isize),
    h: &mut (isize, isize),
    ret: &mut BTreeSet<(isize, isize)>,
) -> (isize, isize) {
    let orig = t.clone();
    match change {
        (0, n) => (0..n.abs()).into_iter().for_each(|_| {
            if *n > 0 {
                if h.1 > t.1 {
                    t.1 += 1;
                    if h.0 != t.0 {
                        t.0 += h.0 - t.0;
                    }
                }
                h.1 += 1;
            } else {
                if h.1 < t.1 {
                    t.1 -= 1;
                    if h.0 != t.0 {
                        t.0 += h.0 - t.0;
                    }
                }
                h.1 -= 1;
            }
            ret.insert(t.clone());
        }),
        (n, 0) => (0..n.abs()).into_iter().for_each(|_| {
            if *n > 0 {
                if h.0 > t.0 {
                    t.0 += 1;
                    if h.1 != t.1 {
                        t.1 += h.1 - t.1;
                    }
                }
                h.0 += 1;
            } else {
                if h.0 < t.0 {
                    t.0 -= 1;
                    if h.1 != t.1 {
                        t.1 += h.1 - t.1;
                    }
                }
                h.0 -= 1;
            }
            ret.insert(t.clone());
        }),
        _ => unreachable!(),
    }
    (t.0 - orig.0, t.1 - orig.1)
}

pub fn part_one(parsed: &Vec<(isize, isize)>) -> i64 {
    let mut steps = BTreeSet::new();
    let mut hpos = (0, 0);
    let mut tpos = (0, 0);
    parsed.iter().for_each(|change| {
        walk(change, &mut tpos, &mut hpos, &mut steps);
    });
    steps.len() as i64
}

pub fn part_two(parsed: Vec<(isize, isize)>) -> i64 {
    let tails = [(0, 0); 9];
    let mut steps = BTreeSet::new();
    let mut hpos = (0, 0);
    let mut nextmoves: Vec<(isize, isize)> = parsed;
    tails.iter().for_each(|t| {
        steps.clear();
        nextmoves = nextmoves
            .iter()
            .map(|change| walk(change, &mut t.clone(), &mut hpos, &mut steps))
            .collect();
        hpos = *t;
    });
    steps.len() as i64
}

pub fn main() {
    let data = read_file(9);
    let parsed = parse(&data);
    println!("Part one: {}", part_one(&parsed));
    println!("Part two: {}", part_two(parsed))
}
