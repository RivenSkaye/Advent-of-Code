#![feature(test)]
use aoc2022::common::read_file;

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    x: usize,
    y: usize,
}

#[cfg(debug_assertions)]
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl From<(usize, usize)> for Position {
    fn from(item: (usize, usize)) -> Self {
        Position::new(item.0, item.1)
    }
}
impl Default for Position {
    fn default() -> Self {
        Position::DEFAULT
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
    pub fn next(&self, dir: Direction) -> Self {
        match dir {
            Direction::UP => Position::new(self.x, self.y - 1),
            Direction::RIGHT => Position::new(self.x + 1, self.y),
            Direction::DOWN => Position::new(self.x, self.y + 1),
            Direction::LEFT => Position::new(self.x - 1, self.y),
        }
    }

    pub const DEFAULT: Position = Position { x: 0, y: 0 };
}

pub type Grid = Vec<Vec<Node>>;

#[derive(Debug, Clone, Copy)]
pub struct Node {
    height: u8,
    visited: bool,
    distance: usize,
}
impl Node {
    pub fn cmp(self, other: Self) -> Self {
        if other.distance < self.distance {
            other
        } else {
            self
        }
    }

    pub fn cmp_dist(&mut self, new_dist: usize) {
        self.distance = self.distance.min(new_dist);
    }

    pub fn new(node_height: u8) -> Self {
        Self {
            height: node_height,
            visited: false,
            distance: usize::MAX,
        }
    }

    pub fn can_walk(&self, other: &Self) -> bool {
        self.height > other.height || other.height - self.height < 2
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        other.height == self.height
    }
    fn ne(&self, other: &Self) -> bool {
        other.height != self.height
    }
}

#[inline(always)]
pub fn get_at(grid: &Grid, pos: &Position) -> Node {
    grid[pos.y][pos.x]
}

/// Returns in order: the puzzle Grid, start Position, end Position
pub fn parse(input: &str) -> (Grid, Position, Position) {
    let mut start = Position::default();
    let mut end = Position::default();
    let mut row = 0;
    let mut col = 0;
    (
        input
            .lines()
            .map(|line| {
                if start == Position::DEFAULT || end == Position::DEFAULT {
                    row += 1;
                    col = 0;
                }
                line.as_bytes()
                    .iter()
                    .map(|position| {
                        if start == Position::DEFAULT || end == Position::DEFAULT {
                            col += 1;
                        }
                        match position {
                            b'S' => {
                                start = (col - 1, row - 1).into();
                                Node {
                                    height: b'a',
                                    visited: false,
                                    distance: 0,
                                }
                            }
                            b'E' => {
                                end = (col - 1, row - 1).into();
                                Node::new(b'z')
                            }
                            p => Node::new(*p),
                        }
                    })
                    .collect()
            })
            .collect(),
        start,
        end,
    )
}

pub fn part_one(mut parsed: Grid, start: Position, end: Position) -> i64 {
    let mut to_check = Vec::<Position>::new();
    to_check.push(start);
    let mut items = 1;
    let mut cur_steps = 0;
    while items > 0 {
        for cur in to_check.clone() {
            to_check.remove(0);
            items -= 1;
            if parsed[cur.y][cur.x].visited {
                continue;
            }
            let curnode = get_at(&parsed, &cur);
            if cur.y > 0 {
                if curnode.can_walk(&get_at(&parsed, &cur.next(Direction::UP))) {
                    to_check.push(cur.next(Direction::UP));
                }
            }
            if cur.x > 0 {
                if curnode.can_walk(&get_at(&parsed, &cur.next(Direction::LEFT))) {
                    to_check.push(cur.next(Direction::LEFT));
                }
            }
            if cur.y < parsed.len() - 1 {
                if curnode.can_walk(&get_at(&parsed, &cur.next(Direction::DOWN))) {
                    to_check.push(cur.next(Direction::DOWN));
                }
            }
            if cur.x < parsed[0].len() - 1 {
                if curnode.can_walk(&get_at(&parsed, &cur.next(Direction::RIGHT))) {
                    to_check.push(cur.next(Direction::RIGHT));
                }
            }
            parsed[cur.y][cur.x].cmp_dist(cur_steps);
            parsed[cur.y][cur.x].visited = true;
            if end.eq(&cur) {
                to_check.clear();
                items = 0;
                break;
            }
            items = to_check.len();
        }
        cur_steps += 1;
    }
    if get_at(&parsed, &end).visited {
        (cur_steps - 1) as i64
    } else {
        i64::MAX
    }
}

pub fn part_two(parsed: Grid, end: Position) -> i64 {
    (0..parsed.len())
        .map(|y| {
            (0..parsed[0].len())
                .map(|x| {
                    let ret = if parsed[y][x].height == b'a' {
                        part_one(parsed.clone(), Position::new(x, y), end)
                    } else {
                        i64::MAX
                    };
                    ret
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

pub fn main() {
    let data = read_file::<12>();
    let (parsed, start, end) = parse(&data);
    println!("{}", part_one(parsed.clone(), start, end));
    println!("{}", part_two(parsed, end));
}

mod aoc_benching {
    extern crate test;
    use super::*;

    #[bench]
    fn parsebench(b: &mut test::Bencher) {
        let input = read_file::<12>();
        b.iter(|| parse(test::black_box(&input)))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let read = read_file::<12>();
        let input = parse(&read);
        b.iter(|| {
            assert_eq!(
                part_one(test::black_box(input.0.clone()), input.1, input.2),
                447
            )
        })
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let read = read_file::<12>();
        let input = parse(&read);
        b.iter(|| assert_eq!(part_two(test::black_box(input.0.clone()), input.2), 446))
    }
}
