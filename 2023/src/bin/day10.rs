#![feature(test)]
extern crate test;

use std::mem::transmute;

use aoc2023::common;

/// Safely transmute valid map entries (u8) to Pipes, see
/// [faster than lime](https://fasterthanli.me/articles/peeking-inside-a-rust-enum)
/// e.g. let pipe = unsafe { transmute(b'|') };
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Pipe {
    UpDown = b'|',    // xConnec, y +/- 1
    LeftRight = b'-', // x +/- 1, y
    DownRight = b'L', // x + 1, y | | x, y - 1
    DownLeft = b'J',  // x - 1, y | | x, y - 1
    UpLeft = b'7',    // x, y + 1 | | x - 1, y
    UpRight = b'F',   // x + 1, y | | x, y + 1
    NoPipe = b'.',    // Can't step
    Start = b'S',     // Wildcard
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left = 1,
    Right = 2,
    Up = 4,
    Down = 8,
}

#[cfg(debug_assertions)]
const MAP_DIMENSIONS: usize = 10;
#[cfg(not(debug_assertions))]
const MAP_DIMENSIONS: usize = 140;

const ALL_DIRS: [Direction; 4] = [
    Direction::Left,
    Direction::Up,
    Direction::Right,
    Direction::Down,
];

#[derive(Debug, Clone, Copy)]
pub struct Point(usize, usize);

// Take into account that the part 2 test input is 10 x 20 tiles
// but it's still a valid loop
#[cfg(debug_assertions)]
pub type Map = [[Pipe; MAP_DIMENSIONS * 2]; MAP_DIMENSIONS];
#[cfg(not(debug_assertions))]
pub type Map = [[Pipe; MAP_DIMENSIONS]; MAP_DIMENSIONS];

pub fn parse(input: &[u8]) -> (Map, Point) {
    #[cfg(debug_assertions)]
    let mut map: Map = [[Pipe::NoPipe; MAP_DIMENSIONS * 2]; MAP_DIMENSIONS];
    #[cfg(not(debug_assertions))]
    let mut map: Map = [[Pipe::NoPipe; MAP_DIMENSIONS]; MAP_DIMENSIONS];

    let mut start = Point(0, 0);
    input
        .split(|c| b'\n'.eq(c))
        .enumerate()
        .for_each(|(y, line)| {
            line.iter().enumerate().for_each(|(x, &chr)| {
                if b'S' == chr {
                    start.0 = y;
                    start.1 = x;
                }
                // SAFETY: see Pipe
                map[y][x] = unsafe { transmute(chr) };
            })
        });
    (map, start)
}

impl Direction {
    #[inline(always)]
    fn check_step(target: Pipe) -> Option<Pipe> {
        (target != Pipe::NoPipe).then_some(target)
    }

    #[inline(always)]
    fn get_next(self, next: Pipe) -> Option<Self> {
        match (&self, next) {
            // Moving horizontally, this pipe lets us continue straight ahead
            (Self::Left | Self::Right, Pipe::LeftRight) => Some(self),
            // Moving left, this pipe bends up
            (Self::Left, Pipe::DownRight) => Some(Self::Up),
            // Moving left, this pipe bends down
            (Self::Left, Pipe::UpRight) => Some(Self::Down),
            // Moving right, this bends up
            (Self::Right, Pipe::DownLeft) => Some(Self::Up),
            // Moving to the right, this bends down
            (Self::Right, Pipe::UpLeft) => Some(Self::Down),
            // Moving vertically, this pipe lets us continue
            (Self::Up | Self::Down, Pipe::UpDown) => Some(self),
            // Moving up, this bends us left
            (Self::Up, Pipe::UpLeft) => Some(Self::Left),
            // Moving up, this bends us right
            (Self::Up, Pipe::UpRight) => Some(Self::Right),
            // Moving down, left turn
            (Self::Down, Pipe::DownLeft) => Some(Self::Left),
            // moving down, right turn
            (Self::Down, Pipe::DownRight) => Some(Self::Right),
            // If you're feeling down, I can feel you up
            _ => None,
        }
    }

    pub fn next_coords(self, pos: &Point) -> Option<Point> {
        match self {
            Self::Left if pos.1 != 0 => Some(Point(pos.0, pos.1 - 1)),
            #[cfg(debug_assertions)]
            Self::Right if pos.1 != (MAP_DIMENSIONS * 2) - 1 => Some(Point(pos.0, pos.1 + 1)),
            #[cfg(not(debug_assertions))]
            Self::Right if pos.1 != MAP_DIMENSIONS - 1 => Some(Point(pos.0, pos.1 + 1)),
            Self::Up if pos.0 != 0 => Some(Point(pos.0 - 1, pos.1)),
            Self::Down if pos.0 != MAP_DIMENSIONS - 1 => Some(Point(pos.0 + 1, pos.1)),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn step(self, pos: &Point, map: &Map) -> Option<(Self, Point)> {
        let next_pos = self.next_coords(pos)?;
        // Check step returns None if the next pipe in this direction is a wall
        // get_next returns None if the next pipe is valid but can't be entered
        // If a direction comes back, provide the next point to check
        Self::check_step(map[next_pos.0][next_pos.1])
            .and_then(|pipe| self.get_next(pipe).map(|dir| (dir, next_pos)))
    }
}

pub fn part_one((map, start_pos): &(Map, Point)) -> usize {
    ALL_DIRS
        .iter()
        // Find the first entrance to the loop; there should only be 2 openings
        .find_map(|&direction| {
            let mut pos = start_pos.to_owned();
            let mut dir = direction;
            let mut steps = 0;
            while let Some((d, p)) = dir.step(&pos, &map) {
                pos = p;
                dir = d;
                steps += 1;
            }
            match dir.next_coords(&pos)? {
                Point(y, x) if map[y][x] == Pipe::Start => Some(steps + 1),
                _ => None,
            }
        })
        .unwrap()
        / 2
}

pub fn part_two((map, start_pos): &(Map, Point)) -> usize {
    // Cheap optimization: skip checking any rows and columns outside of the loop's outer bounds
    let mut min_y = usize::MAX;
    let mut max_y = usize::MIN;
    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    0
}

pub fn main() {
    let data = common::read_file::<10>();
    let parsed = parse(&data);
    // The last test input for p2 produces a result of 80
    println!("{}", part_one(&parsed));
    //println!("{}", part_two(&parsed));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = common::read_file::<10>();
        b.iter(|| parse(test::black_box(&input)))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_file::<10>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part_one(test::black_box(&parsed)), 6842))
    }

    /*#[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_file::<10>();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part_two(test::black_box(&parsed)), 928))
    }*/
}
