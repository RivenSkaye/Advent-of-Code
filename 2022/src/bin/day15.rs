#![feature(test)]

use aoc2022::common::read_file;

#[inline(always)]
pub fn stosi(s: &[u8]) -> i64 {
    match s[0] == b'-' {
        true => s[1..].iter().fold(0, |i, c| (10 * i) - (c - b'0') as i64),
        false => s.iter().fold(0, |i, c| (10 * i) + (c - b'0') as i64),
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point(i64, i64);

#[derive(Copy, Clone)]
pub struct Sensor {
    range: u64,
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
        .lines()
        .map(|line| {
            let mut l = line.split(':');
            Sensor::new(
                l.next()
                    .map(|chunk| {
                        chunk[12..]
                            .split_once(',')
                            .map(|(s0, s1)| Point(stosi(s0.as_bytes()), stosi(s1[3..].as_bytes())))
                    })
                    .unwrap()
                    .unwrap(),
                l.next()
                    .map(|chunk| {
                        chunk[24..]
                            .split_once(',')
                            .map(|(b0, b1)| Point(stosi(b0.as_bytes()), stosi(b1[3..].as_bytes())))
                    })
                    .unwrap()
                    .unwrap(),
            )
        })
        .collect()
}
#[cfg(not(debug_assertions))]
const TARGET: i64 = 2000000;
#[cfg(debug_assertions)]
const TARGET: i64 = 10;

#[inline]
pub fn part_one(parsed: Vec<Sensor>) -> i64 {
    let mut covered = (0, 0);
    // We'll need to keep track of all beacons on the target line, as they're valid
    // beacon positions and need to be subtracted from the amount of scanned positions.
    let mut beacons = Vec::with_capacity(5);
    parsed.iter().for_each(|sensor| {
        // The scanning width will decrease by 2 for every Y point difference,
        // the maximum width is (range * 2) + 1, as it scans its Manhattan distance
        // in all directions, meaning its own X coordinate doesn't fall from the total
        // for its entire range.
        let dist = sensor.point.1.abs_diff(TARGET);
        if dist < sensor.range + 1 {
            if sensor.beaconpos.1 == TARGET && !beacons.contains(&sensor.beaconpos) {
                beacons.push(sensor.beaconpos)
            }
            let min = sensor.point.0 - (sensor.range - dist) as i64;
            let max = sensor.point.0 + (sensor.range - dist) as i64;
            covered.0 = min.min(covered.0);
            covered.1 = max.max(covered.1);
        }
    });
    ((covered.0.abs_diff(covered.1)) - beacons.len() as u64) as i64 + 1
}

#[cfg(not(debug_assertions))]
const MAXRANGE: i64 = 4000001;
#[cfg(debug_assertions)]
const MAXRANGE: i64 = 21;

#[inline(always)]
pub fn in_range(x: i64, y: i64, sensor: &Sensor) -> bool {
    (x.abs_diff(sensor.point.0) + y.abs_diff(sensor.point.1)) < sensor.range + 1
}

#[inline]
pub fn part_two(mut parsed: Vec<Sensor>) -> i64 {
    parsed.sort_unstable_by(|l, r| r.point.0.cmp(&l.point.0));
    let mut x = MAXRANGE - 1;
    loop {
        let mut y = 0;
        'yloop: while y < MAXRANGE {
            if let Some(sensor) = parsed.iter().find(|sensor| in_range(x, y, sensor)) {
                y = ((sensor.point.1 as u64 + sensor.range) - sensor.point.0.abs_diff(x)) as i64
                    + 1;
                continue 'yloop;
            }
            return (x * 4000000) + y;
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
