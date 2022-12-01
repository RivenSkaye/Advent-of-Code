use std::fs;

pub fn read_file(day: usize) -> String {
    let dir = if cfg!(debug_assertions) {
        "test_inputs"
    } else {
        "inputs"
    };
    println!("{dir}");
    fs::read_to_string(format!("{dir}/{day:0>2}.txt")).unwrap()
}
