#![feature(vec_push_within_capacity, test)]
extern crate test;

use aoc2024::common;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Direction {
    UP = 1,    //0b0001,
    DOWN = 2,  //0b0010,
    LEFT = 4,  //0b0100,
    RIGHT = 8, //0b1000,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::UP,
    Direction::LEFT,
    Direction::DOWN,
    Direction::RIGHT,
];

#[derive(Clone)]
pub struct Grid {
    inner: Vec<u8>,
    line_length: usize,
}

impl Grid {
    fn step(&self, from: usize, dir: Direction) -> Option<usize> {
        let next = match (from, dir) {
            (n, Direction::UP) => (n >= self.line_length).then(|| n - self.line_length),
            (n, Direction::LEFT) => (n % self.line_length > 0).then(|| n - 1),
            (n, Direction::RIGHT) => ((n + 1) % self.line_length > 0).then(|| n + 1),
            (n, Direction::DOWN) => {
                (n + self.line_length < self.inner.len()).then(|| n + self.line_length)
            }
        }?;
        // None means a perimeter
        (self.inner[next] == self.inner[from]).then_some(next)
    }

    #[inline(always)]
    fn position(&self, idx: usize) -> (usize, usize) {
        (idx % self.line_length, idx / self.line_length)
    }
}

pub fn parse(input: &[u8]) -> Grid {
    Grid {
        line_length: input.iter().position(|c| b'\n'.eq(c)).unwrap(),
        inner: input
            .iter()
            .filter_map(|c| b'\n'.ne(c).then(|| *c))
            .collect(),
    }
}

fn walk(
    grid: &Grid,
    current: usize,
    area: &mut usize,
    perims: &mut Vec<((usize, usize), Direction)>,
    checked: &mut Vec<usize>,
) {
    if checked.contains(&current) {
        return;
    }
    checked.push(current);
    *area += 1;

    DIRECTIONS.iter().for_each(|&dir| {
        if let Some(next) = grid.step(current, dir) {
            walk(grid, next, area, perims, checked);
        } else {
            perims.push((grid.position(current), dir));
        }
    });
}

pub fn part_one(grid: &Grid) -> usize {
    let mut checked = Vec::with_capacity(grid.inner.len());
    let mut perimeters = Vec::with_capacity(grid.inner.len());
    let mut price = 0;
    for i in 0..grid.inner.len() {
        if checked.contains(&i) {
            continue;
        }
        let mut area = 0;
        walk(grid, i, &mut area, &mut perimeters, &mut checked);
        price += area * perimeters.len();
        perimeters.clear();
    }
    price
}

pub fn part_two(grid: &Grid) -> usize {
    let mut checked = Vec::with_capacity(grid.inner.len());
    let mut perimeters = Vec::with_capacity(grid.inner.len());
    let mut price = 0;
    for i in 0..grid.inner.len() {
        if checked.contains(&i) {
            continue;
        }
        let mut area = 0;
        walk(grid, i, &mut area, &mut perimeters, &mut checked);
        let mut walls = 0;
        // THe walk setup adds both separately so they're never in the same iteration
        for p in 0..perimeters.len() {
            let ((x, y), dir) = perimeters[p];
            // Find vertical walls
            if (dir == Direction::UP || dir == Direction::DOWN)
                && perimeters
                    .iter()
                    .find(|((other_x, other_y), other_dir)| {
                        // check if the square right next to us already has a wall registered
                        y.eq(other_y) && (*other_x + 1) == x && dir.eq(other_dir)
                    })
                    // and if so, do nothing because we already found this wall
                    .is_none()
            {
                walls += 1;
            }
            // Find horizontal walls
            else if (dir == Direction::LEFT || dir == Direction::RIGHT)
                && perimeters
                    .iter()
                    .find(|((other_x, other_y), other_dir)| {
                        // check if the square right above us already has a wall registered
                        x.eq(other_x) && (*other_y + 1) == y && dir.eq(other_dir)
                    })
                    // and if so, do nothing because we already found this wall
                    .is_none()
            {
                walls += 1;
            }
        }
        price += area * walls;
        perimeters.clear();
    }
    price
}

pub fn main() {
    let input = common::read_file::<12>();
    let parsed = parse(&input);
    println!("{}", part_one(&parsed));
    println!("{}", part_two(&parsed));
}
