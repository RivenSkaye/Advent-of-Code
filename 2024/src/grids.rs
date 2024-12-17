#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

pub struct FlatGrid {
    inner: Vec<u8>,
    line_length: usize,
}

impl<T> From<&[u8]> for FlatGrid {
    fn from(value: &[u8]) -> Self {
        Self {
            line_length: value.iter().position(|c| b'\n'.eq(c)).unwrap(),
            inner: value.iter().filter(|chr| b'\n'.ne(chr)).collect(),
        }
    }
}

impl FlatGrid {
    #[inline(always)]
    pub fn position(&self, idx: usize) -> (usize, usize) {
        (position % self.line_length, position / self.line_length)
    }

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
