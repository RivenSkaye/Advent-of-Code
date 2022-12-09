#[cfg(debug_assertions)]
const FILENAME: &str = "test_inputs";
#[cfg(not(debug_assertions))]
const FILENAME: &str = "inputs";

#[inline(always)]
pub fn read_file<const DAY: usize>() -> String {
    println!("Reading `{FILENAME}`");
    unsafe { std::fs::read_to_string(format!("{FILENAME}/{DAY:0>2}.txt")).unwrap_unchecked() }
}
