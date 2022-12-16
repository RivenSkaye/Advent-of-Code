#[cfg(debug_assertions)]
const FILENAME: &str = "test_inputs";
#[cfg(not(debug_assertions))]
const FILENAME: &str = "inputs";

#[inline(always)]
pub fn read_file<const DAY: usize>() -> String {
    println!("Reading `{FILENAME}`");
    std::fs::read_to_string(format!("{FILENAME}/{DAY:0>2}.txt")).unwrap()
}

#[inline(always)]
pub fn stoi(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .fold(0, |i, c| (10 * i) + (c - b'0') as usize)
}

#[inline(always)]
pub fn stosi(s: &str) -> isize {
    match s.starts_with('-') {
        true => -(stoi(&s.replace('-', "")) as isize),
        false => stoi(s) as isize,
    }
}
