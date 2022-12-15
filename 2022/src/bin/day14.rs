use aoc2022::common::{read_file, stoi};
use std::iter::repeat_with;

const CAVEHEIGHT: usize = 200;
const CAVEWIDTH: usize = 2 * CAVEHEIGHT - 1;
const X_OFFSET: usize = 500 - CAVEWIDTH / 2;
const SOURCE: usize = 500 - X_OFFSET;
type Grid = [[u8; CAVEWIDTH]; CAVEHEIGHT];

pub fn parse(input: &str) -> (Grid, usize, usize) {
    let mut cave: Grid = [[b'.'; CAVEWIDTH]; CAVEHEIGHT];
    let mut maxy = 0;
    let mut yblock = 0;
    input.lines().for_each(|line| {
        let mut x = 0;
        let mut y = 0;
        line.split(" -> ").for_each(|pair| {
            let (newx, newy) = pair
                .split_once(',')
                .map(|n| (stoi(n.0) - X_OFFSET, stoi(n.1)))
                .unwrap();
            if x > 0 || y > 0 {
                match (newx != x, newy != y) {
                    (true, _) => (newx.min(x)..=newx.max(x)).into_iter().for_each(|xval| {
                        if xval == 500 && yblock == 0 {
                            yblock = newy - 1;
                        }
                        cave[newy][xval] = b'#'
                    }),

                    (_, true) => (newy.min(y)..=newy.max(y))
                        .into_iter()
                        .for_each(|yval| cave[yval][newx] = b'#'),

                    _ => unreachable!(),
                }
            }
            (x, y) = (newx, newy);
            if y > maxy {
                maxy = y;
            }
        })
    });
    cave[maxy + 2].iter_mut().for_each(|space| *space = b'#');
    (cave, yblock, maxy)
}

pub fn sand_fall(cave: &mut Grid, x: usize, y: usize, ymax: usize) -> bool {
    if y == ymax {
        false
    } else if cave[y + 1][x] == b'.' {
        sand_fall(cave, x, y + 1, ymax)
    } else if cave[y + 1][x - 1] == b'.' {
        sand_fall(cave, x - 1, y + 1, ymax)
    } else if cave[y + 1][x + 1] == b'.' {
        sand_fall(cave, x + 1, y + 1, ymax)
    } else {
        cave[y][x] = b'o';
        if x == SOURCE && y == 0 {
            return false;
        }
        true
    }
}

pub fn part_one(cave: &mut Grid, yfirst: usize, ymax: usize) -> i64 {
    // We count how many grains have fallen, but we only need to know how many
    // fell **before** they went into the abyss
    let mut grains = -1;
    repeat_with(|| {
        grains += 1;
        sand_fall(cave, SOURCE, yfirst, ymax)
    })
    .take_while(|fall| *fall)
    .for_each(drop);
    grains
}

pub fn part_two(cave: &mut Grid, ymax: usize) -> i64 {
    // There is no iteration falling into the abyss anymore!
    let mut grains = 0;
    repeat_with(|| {
        grains += 1;
        sand_fall(cave, SOURCE, 0, ymax + 2)
    })
    .take_while(|fall| *fall)
    .for_each(drop);
    grains
}

pub fn main() {
    let data = read_file::<14>();
    let (mut parsed, yfirst, ymax) = parse(&data);
    println!("{}", part_one(&mut parsed.clone(), yfirst, ymax));
    println!("{}", part_two(&mut parsed, ymax));
    parsed.iter().for_each(|row| {
        row.iter().for_each(|c| print!("{}", *c as char));
        println!("");
    });
}
