#![feature(array_windows, test)]

use aoc2024::{common, grids::*};

#[derive(Clone)]
pub struct Grid(FlatGrid);
impl<T: Deref<Target = [u8]>> From<T> for Grid {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl Grid {
    fn step(&mut self, current: usize, dir: Direction) -> Option<usize> {
        let grid = &mut self.0;
        let next = grid.bounded_step(current, dir).unwrap();
        if grid.inner[next] == b'#' {
            return None;
        }
        if grid.inner[next] == b'O' {
            let mut bound = next;
            loop {
                bound = grid.bounded_step(bound, dir).unwrap();
                if grid.inner[bound] == b'.' {
                    grid.inner.swap(bound, next);
                    break;
                }
                if grid.inner[bound] == b'#' {
                    return None;
                }
            }
        }
        if grid.inner[next] == b'.' {
            grid.inner.swap(next, current);
            return Some(next);
        }
        None
    }

    fn position(&self, idx: usize) -> (usize, usize) {
        self.0.position(idx)
    }

    #[allow(dead_code)]
    fn print(&self) {
        for i in 0..self.0.inner.len() {
            print!("{}", String::from_utf8_lossy(&self.0.inner[i..=i]));
            if (i + 1) % self.0.line_length == 0 {
                println!("");
            }
        }
    }
}

#[inline(always)]
pub fn parse(input: &[u8]) -> (usize, Grid, Vec<Direction>) {
    let split = input
        .array_windows()
        .position(|[a, b]| a == b && b'\n'.eq(a))
        .unwrap();
    let grid = Grid::from(&input[..split]);
    (
        grid.0.inner.iter().position(|chr| b'@'.eq(chr)).unwrap(),
        grid,
        input[split..]
            .iter()
            .filter_map(|chr| match chr {
                b'^' => Some(Direction::UP),
                b'>' => Some(Direction::RIGHT),
                b'v' => Some(Direction::DOWN),
                b'<' => Some(Direction::LEFT),
                _ => None,
            })
            .collect(),
    )
}

pub fn part_one(mut grid: Grid, mut start: usize, instructions: &[Direction]) -> usize {
    for &direction in instructions {
        start = grid.step(start, direction).unwrap_or(start);
    }
    let res = grid
        .0
        .inner
        .iter()
        .enumerate()
        .filter_map(|(idx, space)| {
            b'O'.eq(space)
                .then(|| grid.position(idx))
                .map(|(x, y)| (100 * y) + x)
        })
        .sum();
    res
}

pub fn main() {
    let input = common::read_file::<15>();
    let (start, grid, instructions) = parse(&input);
    println!("{}", part_one(grid.clone(), start, &instructions))
}
