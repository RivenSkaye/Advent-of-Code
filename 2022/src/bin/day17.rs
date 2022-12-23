use aoc2022::common::read_file;

#[inline(always)]
pub fn parse(input: &str) -> &[u8] {
    // Strip off the trailing newline and give back an owned Vec
    &input[0..input.len() - 1].as_bytes()
}

const DASH: [(usize, usize); 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
const PLUS: [(usize, usize); 5] = [(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)];
const ANGLE: [(usize, usize); 5] = [(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)];
const PIPE: [(usize, usize); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
const SQUARE: [(usize, usize); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];

#[inline]
pub fn shapewidth(shape: &[(usize, usize)]) -> isize {
    shape.iter().map(|&(_, x)| x).max().unwrap() as isize + 1
}

#[inline]
pub fn match_shape<'a>(stone: usize) -> &'a [(usize, usize)] {
    match stone % 5 {
        0 => &DASH,
        1 => &PLUS,
        2 => &ANGLE,
        3 => &PIPE,
        4 => &SQUARE,
        _ => unreachable!(),
    }
}

#[inline]
pub fn get_jet(input: &[u8], num: usize) -> isize {
    match input[num % input.len()] {
        b'<' => -1,
        b'>' => 1,
        _ => unreachable!(),
    }
}

/// Function that modifies a window in-place to shift all rows by one until
/// the first row where a block can land on the window floor.
/// This allows for the window to be as small as 2x the highest block, which
/// means this function only needs to be called if the next block might exceed
/// the maximum window height.For this AoC day, that's `2 * 4 = 8`.<br />
/// Heavily abuses the fact I mark unreachable blocks as solid rock.
///
/// Returns the amount of rows shifted upwards.
#[inline]
pub fn move_window(window: &mut [[u8; 7]], width: usize) -> usize {
    let mut rows = 0;
    while window[1].iter().filter(|b| b'.'.eq(b)).count() < width {
        for r in 1..window.len() - 1 {
            window[r] = window[r + 1];
            if window[r + 1] == [b'.'; 7] {
                break;
            }
            window[r + 1] = [b'.'; 7];
        }
        rows += 1;
    }
    rows
}

pub fn sim<const STONES: usize>(input: &[u8]) -> i64 {
    let mut stacks = [0; 7];
    let mut next_stone: &[(usize, usize)];
    let mut jetcount = 0;
    // Set to 4: 3 above the dash.
    for stone in 0..STONES {
        next_stone = match_shape(stone);
        let mut blockheight = *stacks.iter().max().unwrap() + 1;
        // Three spaces of free fall, always
        let mut x_offset = (jetcount..jetcount + 3)
            .into_iter()
            .map(|j| get_jet(input, j))
            .fold(2, |cur, xo| (cur + xo).clamp(0, 7 - shapewidth(next_stone)))
            as usize;
        jetcount += 3;
        loop {
            let jet = get_jet(input, jetcount);
            jetcount += 1;
            // Shift with the jet if possible
            let xo = (x_offset as isize + jet).clamp(0, 7 - shapewidth(next_stone)) as usize;
            if next_stone
                .iter()
                .map(|(y, x)| (blockheight + y) > stacks[xo + x])
                .all(|b| b)
            {
                x_offset = xo;
            }
            // Then check for the drop
            if next_stone
                .iter()
                .map(|(y, x)| (blockheight + y - 1) > stacks[x_offset + x])
                .all(|b| b)
            {
                blockheight -= 1;
            } else {
                for i in 0..shapewidth(next_stone) as usize {
                    stacks[x_offset + i] = next_stone
                        .iter()
                        .filter(|(_, x)| i.eq(x))
                        .map(|(y, _)| y + blockheight)
                        .max()
                        .unwrap();
                }
                break;
            }
        }
    }
    *stacks.iter().max().unwrap() as i64
}

pub fn main() {
    let input = read_file::<17>();
    println!("{}", sim::<2022>(parse(&input)));
}
