pub use std::ops::Deref;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    #[inline]
    pub fn turn_clock(&self) -> Self {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }

    #[inline]
    pub fn turn_counter(&self) -> Self {
        match self {
            Direction::UP => Direction::LEFT,
            Direction::RIGHT => Direction::UP,
            Direction::DOWN => Direction::RIGHT,
            Direction::LEFT => Direction::DOWN,
        }
    }

    #[inline]
    pub fn reverse(&self) -> Self {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::RIGHT => Direction::LEFT,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
        }
    }
}

#[derive(Clone)]
pub struct FlatGrid {
    pub inner: Vec<u8>,
    pub line_length: usize,
}

impl<T: Deref<Target = [u8]>> From<T> for FlatGrid {
    fn from(value: T) -> Self {
        Self {
            line_length: value.iter().position(|c| b'\n'.eq(c)).unwrap(),
            inner: value
                .iter()
                .filter_map(|chr| b'\n'.ne(chr).then_some(*chr))
                .collect(),
        }
    }
}

impl FlatGrid {
    #[inline(always)]
    pub fn position(&self, idx: usize) -> (usize, usize) {
        (idx % self.line_length, idx / self.line_length)
    }

    #[inline]
    pub fn bounded_step(&self, current: usize, dir: Direction) -> Option<usize> {
        match (current, dir) {
            (n, Direction::UP) => (n >= self.line_length).then(|| n - self.line_length),
            (n, Direction::LEFT) => (n % self.line_length > 0).then(|| n - 1),
            (n, Direction::RIGHT) => ((n + 1) % self.line_length > 0).then(|| n + 1),
            (n, Direction::DOWN) => {
                (n + self.line_length < self.inner.len()).then(|| n + self.line_length)
            }
        }
    }

    #[inline]
    pub fn unbounded_step(&self, current: usize, dir: Direction) -> usize {
        match dir {
            Direction::UP => current - self.line_length,
            Direction::DOWN => current + self.line_length,
            Direction::LEFT => current - 1,
            Direction::RIGHT => current + 1,
        }
    }

    #[inline]
    pub fn pacman_step(&self, current: usize, dir: Direction) -> usize {
        match (current, dir) {
            (n, Direction::UP) => {
                if n < self.line_length {
                    (self.inner.len() - self.line_length) + n
                } else {
                    n - self.line_length
                }
            }
            (n, Direction::LEFT) => {
                if n % self.line_length > 0 {
                    n - 1
                } else {
                    (n - 1) + self.line_length
                }
            }
            (n, Direction::RIGHT) => {
                if (n + 1) % self.line_length > 0 {
                    n + 1
                } else {
                    (n + 1) - self.line_length
                }
            }
            (n, Direction::DOWN) => {
                if (n + self.line_length) >= self.inner.len() {
                    n % self.line_length
                } else {
                    n + self.line_length
                }
            }
        }
    }
}
