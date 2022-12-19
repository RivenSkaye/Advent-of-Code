#![feature(test)]
use std::{collections::HashSet, ops::Sub};

use aoc2022::common::{read_file, stosi};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point(isize, isize);
impl From<(isize, isize)> for Point {
    fn from((x, y): (isize, isize)) -> Self {
        Self(x, y)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Sensor {
    range: usize,
    point: Point,
    beaconpos: Point,
}

impl Sensor {
    pub fn new(sensor: Point, beacon: Point) -> Self {
        Sensor {
            range: sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1),
            point: sensor,
            beaconpos: beacon,
        }
    }
}

pub fn parse(input: &str) -> Vec<Sensor> {
    // Strip off all the useless garbo, it's noise.
    // That leaves us with xxx,yyy:xxx,yyy points!
    input
        .replace("Sensor at x=", "")
        .replace(" closest beacon is at x=", "")
        .replace(" y=", "")
        .lines()
        .map(|line| {
            let mut sensor = (0, 0);
            let mut beacon = (0, 0);
            let mut counter = 0;
            line.split(':').for_each(|chunk| {
                chunk.split(',').for_each(|n| {
                    match counter % 4 {
                        0 => sensor.0 = stosi(n),
                        1 => sensor.1 = stosi(n),
                        2 => beacon.0 = stosi(n),
                        3 => beacon.1 = stosi(n),
                        _ => unreachable!(),
                    }
                    counter += 1;
                });
            });
            Sensor::new(sensor.into(), beacon.into())
        })
        .collect()
}
#[cfg(not(debug_assertions))]
const TARGET: isize = 2000000;
#[cfg(debug_assertions)]
const TARGET: isize = 10;

#[cfg(not(debug_assertions))]
const MAXRANGE: i64 = 4000001;
#[cfg(debug_assertions)]
const MAXRANGE: i64 = 20;

pub fn part_one(mut parsed: Vec<Sensor>) -> i64 {
    parsed.sort_unstable_by(|l, r| {
        let ldiff = l.point.1.abs_diff(TARGET);
        let rdiff = r.point.1.abs_diff(TARGET);
        if ldiff > l.range && rdiff > r.range {
            std::cmp::Ordering::Equal
        } else if ldiff > l.range {
            std::cmp::Ordering::Greater
        } else if rdiff > r.range {
            std::cmp::Ordering::Less
        } else {
            (l.range - ldiff).cmp(&(r.range - rdiff))
        }
    });
    // We know we'll cover a maximum amount of ranges equal to the amount of sensors
    // though most cases where a sensor doesn't cover the target line, we won't
    // need to consider it, so we'll be using less than allocated. Single alloc gang!
    let mut covered = Vec::<(isize, isize)>::with_capacity(parsed.len());
    // We'll need to keep track of all beacons on the target line, as they're valid
    // beacon positions and need to be subtracted from the amount of scanned positions.
    let mut beacons = HashSet::with_capacity(parsed.len());
    parsed.iter().for_each(|sensor| {
        // The scanning width will decrease by 2 for every Y point difference,
        // the maximum width is (range * 2) + 1, as it scans its Manhattan distance
        // in all directions, meaning its own X coordinate doesn't fall from the total
        // for its entire range.
        let dist = sensor.point.1.abs_diff(TARGET);
        if dist < sensor.range + 1 {
            if sensor.beaconpos.1 == TARGET {
                beacons.insert(sensor.beaconpos);
            }
            let min = sensor.point.0 - (sensor.range - dist) as isize;
            let max = sensor.point.0 + (sensor.range - dist) as isize;
            let mut push = true;
            for i in 0..covered.len() {
                // If this max lies within any covered range and the min is lower,
                // just expand the range as we have overlap
                if covered[i].0 > min - 1 && covered[i].1 > max - 1 && covered[i].0 < max + 1 {
                    covered[i].0 = min;
                    push = false;
                    break;
                }
                // Same check for the min inside and the max outside
                if covered[i].1 < max + 1 && covered[i].0 < min + 1 && covered[i].1 > min - 1 {
                    covered[i].1 = max;
                    push = false;
                    break;
                }
                // If this range contains an existing range entirely, grow the existing range
                if covered[i].0 > min - 1 && covered[i].1 < max + 1 {
                    covered[i] = (min, max);
                    push = false;
                    break;
                }
                // Ignore the range if it's entirely contained in another
                if covered[i].0 < min + 1 && covered[i].1 > max - 1 {
                    push = false;
                }
            }
            if push {
                covered.push((min, max));
            }
        }
    });
    for i in 0..covered.len() {
        for j in 0..covered.len() {
            if i == j {
                continue;
            }
            // If we find overlap
            if covered[i].1 >= covered[j].0 && covered[i].0 <= covered[j].0 {
                // truncate the range with the higher starting number
                covered[j].0 = covered[i].1 + 1;
            }
        }
    }
    covered
        .iter()
        .map(|covrange| {
            if covrange.0 > covrange.1 {
                0
            } else {
                // 3.abs_diff(5) = 2. But if a range contains 3..=5, it's 3 positions
                (covrange.0.abs_diff(covrange.1) + 1) as i64
            }
        })
        .sum::<i64>()
        .sub(beacons.len() as i64)
}

#[inline(always)]
pub fn in_range(point: Point, sensor: &Sensor) -> bool {
    (point.0.abs_diff(sensor.point.0) + point.1.abs_diff(sensor.point.1)) < sensor.range + 1
}

pub fn part_two(parsed: Vec<Sensor>) -> i64 {
    let mut x = MAXRANGE - 1;
    loop {
        let mut y: i64 = 0;
        while y < MAXRANGE {
            match &parsed
                .iter()
                .find(|sensor| in_range(Point(x as isize, y as isize), sensor))
            {
                Some(sensor) => {
                    y = ((sensor.point.1 as usize + sensor.range)
                        - sensor.point.0.abs_diff(x as isize)) as i64
                        + 1;
                }
                _ => return (x * 4000000) + y,
            }
        }
        x -= 1;
    }
}

pub fn main() {
    let data = read_file::<15>();
    let parsed = parse(&data);
    println!("{}", part_one(parsed.clone()));
    println!("{}", part_two(parsed));
}

mod aoc_benching {
    extern crate test;
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = read_file::<15>();
        b.iter(|| parse(test::black_box(&input)))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let read = read_file::<15>();
        let parsed = parse(&read);
        b.iter(|| assert_eq!(part_one(test::black_box(parsed.clone())), 5125700))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let read = read_file::<15>();
        let parsed = parse(&read);
        b.iter(|| assert_eq!(part_two(test::black_box(parsed.clone())), 11379394658764));
    }
}
