#![feature(int_roundings)]
use aoc2022::common::read_file;

pub struct Monkey {
    items: Vec<u64>,
    operation: fn(u64, u64) -> u64,
    opnumber: u64,
    testdiv: u64,
    truemonkey: usize,
    falsemonkey: usize,
    inspectcounter: usize,
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let inv = self
            .items
            .iter()
            .map(|it| it.to_string())
            .collect::<Vec<String>>();
        write!(f, "Monkey\n  Items: {}\n  Operation: old [+|*] {}\n  Test: mod {}\n    True => {}\n    False => {}",
               inv.join(", "),
               self.opnumber,
               self.testdiv,
               self.truemonkey,
               self.falsemonkey
        )
    }
}

impl Monkey {
    pub fn test(&mut self) -> Vec<(usize, u64)> {
        let ret = self
            .items
            .iter_mut()
            .map(|worryval| {
                *worryval = worryval.div_floor(3);
                if *worryval % self.testdiv == 0 {
                    (self.truemonkey, *worryval)
                } else {
                    (self.falsemonkey, *worryval)
                }
            })
            .collect();
        self.items.clear();
        ret
    }

    pub fn inspect(&mut self) {
        self.inspectcounter += self.items.len();
        self.items.iter_mut().for_each(|item| {
            *item = (self.operation)(*item, self.opnumber);
        })
    }
}

pub fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey| {
            let mnk = monkey.as_bytes();
            // Start at the beginning of its items
            let mut i = 28;
            // init a vec for it
            let mut inv = Vec::new();
            // loop time!
            loop {
                // Assume all items have a base value of 2 digits. Yolotime!
                inv.push(((mnk[i] - b'0') * 10 + (mnk[i + 1] - b'0')) as u64);
                i += 2;
                // Check what follows the item's value
                match mnk[i] {
                    // comma denotes another item. Skip over the comma and the space, repeat loop
                    b',' => i += 2,
                    // newline denotes the end of the items.
                    b'\n' => {
                        // skip to the test operation numbers
                        i += 26;
                        break;
                    }
                    _ => unreachable!(),
                }
            }
            // grab the operation, + or *
            let op = mnk[i - 2];
            let stressop: fn(u64, u64) -> u64 = match op {
                b'*' => |x, y| if y == 0 { x * x } else { x * y },
                _ => |x, y| if y == 0 { x + x } else { x + y },
            };
            // check the operation
            let opnum = match (mnk[i], mnk[i + 1]) {
                // if it starts with "o" it's "old"
                (b'o', _) => {
                    i += 2;
                    0
                }
                // every other case is a number followed by either another number, or a newline.
                (n, b'\n') => (n - b'0') as u64,
                (n, m) => {
                    i += 1;
                    ((n - b'0') * 10 + (m - b'0')) as u64
                }
            };
            // Skip to the 1 or 2 digit test divisor and capture it
            i += 23;
            let divisor = match mnk[i + 1] {
                b'\n' => (mnk[i] - b'0') as u64,
                _ => {
                    i += 1;
                    ((mnk[i - 1] - b'0') * 10 + (mnk[i] - b'0')) as u64
                }
            };
            // skip to the index of the true monkey. There are less than 11 (0-9), more yolomode
            i += 31;
            let truemonk = (mnk[i] - b'0') as usize;
            // Rinse and repeat for the false monkey
            i += 32;
            let falsemonk = (mnk[i] - b'0') as usize;
            Monkey {
                items: inv,
                operation: stressop,
                opnumber: opnum,
                testdiv: divisor,
                truemonkey: truemonk,
                falsemonkey: falsemonk,
                inspectcounter: 0,
            }
        })
        .collect()
}

#[inline]
pub fn round(monkeys: &mut Vec<Monkey>) -> (usize, usize) {
    let mut max2 = (0, 0);
    for i in 0..monkeys.len() {
        monkeys[i].inspect();
        monkeys[i]
            .test()
            .iter()
            .for_each(|(throw_to, worry)| unsafe {
                monkeys.get_unchecked_mut(*throw_to).items.push(*worry)
            });
        match (
            max2.0 > max2.1,
            monkeys[i].inspectcounter > max2.0,
            monkeys[i].inspectcounter > max2.1,
        ) {
            (true, _, true) => max2.1 = monkeys[i].inspectcounter,
            (false, true, _) => max2.0 = monkeys[i].inspectcounter,
            _ => (),
        }
    }
    max2
}

#[inline]
pub fn mult(vals: (usize, usize)) -> i64 {
    (vals.0 * vals.1) as i64
}

pub fn part_one(mut monks: Vec<Monkey>) -> i64 {
    for _ in 0..19 {
        round(&mut monks);
    }
    mult(round(&mut monks))
}

pub fn main() {
    let data = read_file::<11>();
    let parsed = parse(&data);
    println!("{}", part_one(parsed));
}
