#![feature(test)]
extern crate test;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use std::collections::HashMap;

use aoc2023::common;

pub fn parse(input: &str)

pub fn main() {}
