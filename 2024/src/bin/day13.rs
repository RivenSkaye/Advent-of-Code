#![feature(iter_array_chunks, test)]
extern crate test;

use aoc2024::common;

pub fn parse(input: &[u8]) -> Vec<(isize, isize, isize, isize, isize, isize)> {
    input
        .split(|chr| b'\n'.eq(chr))
        .filter(|line| !line.is_empty())
        .array_chunks::<3>()
        .map(|[a, b, p]| {
            let asplit = a.iter().position(|chr| b','.eq(chr)).unwrap();
            let bsplit = b.iter().position(|chr| b','.eq(chr)).unwrap();
            let psplit = p.iter().position(|chr| b','.eq(chr)).unwrap();
            // Casting these is faster than checking for a sign - gotta love input assumptions
            let ax = common::stoi(&a[12..asplit]) as isize;
            let ay = common::stoi(&a[(asplit + 4)..]) as isize;
            let bx = common::stoi(&b[12..bsplit]) as isize;
            let by = common::stoi(&b[(bsplit + 4)..]) as isize;
            let px = common::stoi(&p[9..psplit]) as isize;
            let py = common::stoi(&p[(psplit + 4)..]) as isize;
            (ax, ay, bx, by, px, py)
        })
        .collect()
}

/// `vars` might be more aptly named `axvals_ayvals_bxvals_byvals_pxvals_pyvals`
/// but for the sake of brevity I'll keep the name short.
/// This fn was once pub fn part_one without the const arg :)
fn mash_buttons<const ERR: isize>(vars: &[(isize, isize, isize, isize, isize, isize)]) -> isize {
    vars.iter()
        .filter_map(|(ax, ay, bx, by, px, py)| {
            let py = py + ERR;
            let px = px + ERR;
            let divisor = ax * by - bx * ay;
            if divisor != 0 {
                let a_divident = (px * by) - (py * bx);
                // Thanks to FichteFoll for the optimization trick here
                let a_presses = Some(a_divident / divisor).filter(|x| x * divisor == a_divident)?;
                let b_divident = (ax * py) - (ay * px);
                // Thanks to FichteFoll for the optimization trick here
                let b_presses = Some(b_divident / divisor).filter(|x| x * divisor == b_divident)?;
                Some((a_presses * 3) + b_presses)
            } else {
                None
            }
        })
        .sum()
}

pub fn part_one(vars: &[(isize, isize, isize, isize, isize, isize)]) -> isize {
    mash_buttons::<0>(vars)
}

pub fn part_two(vars: &[(isize, isize, isize, isize, isize, isize)]) -> isize {
    mash_buttons::<10000000000000>(vars)
}

pub fn main() {
    let input = common::read_file::<13>();
    let parsed = parse(&input);
    println!("{}", part_one(&parsed));
    println!("{}", part_two(&parsed));
}
