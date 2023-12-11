#[cfg(debug_assertions)]
const FILENAME: &str = "test_inputs";
#[cfg(not(debug_assertions))]
const FILENAME: &str = "inputs";

#[inline(always)]
pub fn read_file<const DAY: usize>() -> Vec<u8> {
    println!("Reading `{FILENAME}`");
    std::fs::read(format!("{FILENAME}/day{DAY:0>2}.txt"))
        .unwrap()
        .trim_ascii()
        .iter()
        .filter_map(|c| b'\r'.ne(c).then_some(*c))
        .collect()
}

#[inline(always)]
pub fn read_str<const DAY: usize>() -> String {
    println!("Reading `{FILENAME}`");
    std::fs::read_to_string(format!("{FILENAME}/day{DAY:0>2}.txt"))
        .unwrap()
        .replace("\r", "")
}

#[inline(always)]
pub fn stoi(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .fold(0, |i, c| (10 * i) + (c - b'0') as usize)
}

#[inline(always)]
pub fn stosi(s: &[u8]) -> i64 {
    match s[0] == b'-' {
        true => s[1..].iter().fold(0, |i, c| (10 * i) - (c - b'0') as i64),
        false => s.iter().fold(0, |i, c| (10 * i) + (c - b'0') as i64),
    }
}
