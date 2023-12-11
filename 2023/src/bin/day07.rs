#![feature(slice_first_last_chunk)]
#![feature(test)]
extern crate test;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use std::cmp::Ordering;

use aoc2023::common;

pub type Hand = ([u8; 5], HandType, usize);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 4,
    ThreeKind = 8,
    FullHouse = 16,
    FourKind = 32,
    FiveKind = 64,
}

#[inline(always)]
fn cards_to_points(cards: &[u8; 5]) -> [u8; 5] {
    let mut newcards = [0; 5];
    for i in 0..5 {
        newcards[i] = match cards[i] {
            b'A' => 13,
            b'K' => 12,
            b'Q' => 11,
            b'J' => 10,
            b'T' => 9,
            c => c - b'1',
        };
    }
    newcards
}

#[inline(always)]
fn get_handtype<const P1: bool>(hand: [u8; 5]) -> HandType {
    let mut counts = [0usize; 13];
    let mut jokers = 0;
    let hand = if P1 {
        hand
    } else {
        // Get a sorted hand if J is a Joker
        let mut h = hand.clone();
        h.sort();
        h
    };
    for item in hand {
        if !P1 && item == 10 {
            jokers += 1;
        } else {
            counts[item as usize - 1] += 1;
        }
    }
    counts.sort();
    if !P1 {
        counts[12] += jokers;
    }
    match counts {
        // 5 w/ all 0s
        [.., 5] => HandType::FiveKind,
        // 4 w/ all 0s and a single 1
        [.., 4] => HandType::FourKind,
        // 3 and 2 or 2 and 3 with all 0s
        [.., 2, 3] => HandType::FullHouse,
        // 3 and two 1s in any order
        [.., 3] => HandType::ThreeKind,
        // 2 and 2, with a single 1
        [.., 2, 2] => HandType::TwoPair,
        // A single 2 with three 1s
        [.., 2] => HandType::OnePair,
        // all unique
        _ => HandType::HighCard,
    }
}

#[inline(always)]
pub fn parse<const P1: bool>(input: &[u8]) -> Vec<Hand> {
    input
        .split(|c| b'\n'.eq(c))
        .map(|line| {
            let cards = cards_to_points(line.first_chunk::<5>().unwrap());
            (
                cards,
                get_handtype::<P1>(cards),
                // skip the space
                line[6..]
                    .iter()
                    .fold(0, |p, cur| (10 * p) + (cur - b'0') as usize),
            )
        })
        .collect()
}

#[inline(always)]
fn get_winnings(hands: &[([u8; 5], HandType, usize)]) -> usize {
    hands
        .iter()
        .zip(1..)
        .map(|(&(_, _, bid), rank)| bid * rank)
        .sum()
}

pub fn part_one(input: &[Hand]) -> usize {
    let mut hands = input.to_vec();
    hands.sort_by(|lhs, rhs| {
        lhs.1.cmp(&rhs.1).then_with(|| {
            lhs.0
                .iter()
                .zip(rhs.0.iter())
                .find_map(|(lcard, rcard)| match lcard.cmp(rcard) {
                    Ordering::Equal => None,
                    ord => Some(ord),
                })
                .unwrap()
        })
    });
    get_winnings(&hands)
}

pub fn part_two(input: &[Hand]) -> usize {
    let mut hands = input.to_vec();
    hands.sort_by(|lhs, rhs| {
        lhs.1.cmp(&rhs.1).then_with(|| {
            lhs.0
                .iter()
                .zip(rhs.0.iter())
                .find_map(|(&lcard, &rcard)| {
                    let l = if lcard == 10 { 0 } else { lcard };
                    let r = if rcard == 10 { 0 } else { rcard };
                    match l.cmp(&r) {
                        Ordering::Equal => None,
                        ord => Some(ord),
                    }
                })
                .unwrap()
        })
    });
    get_winnings(&hands)
}

pub fn main() {
    let data = common::read_file::<7>();
    let parsed1 = parse::<true>(&data);
    println!("{}", part_one(&parsed1));
    let parsed2 = parse::<false>(&data);
    println!("{}", part_two(&parsed2));
}

#[cfg(test)]
mod aoc_benching {
    use super::*;

    #[bench]
    fn parse1bench(b: &mut test::Bencher) {
        let input = common::read_file::<7>();
        let parsed = parse::<true>(&input);
        b.iter(|| assert_eq!(parse::<true>(test::black_box(&input)), parsed))
    }

    #[bench]
    fn part1bench(b: &mut test::Bencher) {
        let input = common::read_file::<7>();
        let parsed = parse::<true>(&input);
        b.iter(|| assert_eq!(part_one(test::black_box(&parsed)), 256448566))
    }

    #[bench]
    fn parse2bench(b: &mut test::Bencher) {
        let input = common::read_file::<7>();
        let parsed = parse::<false>(&input);
        b.iter(|| assert_eq!(parse::<false>(test::black_box(&input)), parsed))
    }

    #[bench]
    fn part2bench(b: &mut test::Bencher) {
        let input = common::read_file::<7>();
        let parsed = parse::<false>(&input);
        b.iter(|| assert_eq!(part_two(test::black_box(&parsed)), 254412181))
    }
}
