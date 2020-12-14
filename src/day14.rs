use std::collections::HashMap;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Eq)]
pub enum Parsed {
    #[display("mask = {0}")]
    Mask(String),
    #[display("mem[{0}] = {1}")]
    Addr(u64, u64),
}

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy, Eq)]
pub enum Val {
    #[display("1")]
    One,
    #[display("0")]
    Zero,
    #[display("X")]
    X,
}

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub struct Mask {
    mask: [Val; 36],
    permutation: u64,
    max_x: u64,
}

impl Mask {
    pub fn next(&mut self) -> Option<(u64, u64)> {
        if self.permutation >= self.max_x {
            return None;
        }
        let mut x_count = 0;
        let mut res_and = 0;
        let mut res_or = 0;
        for i in 0..36 {
            match (self.mask[35 - i], self.permutation & (1 << x_count) != 0) {
                (Val::One, _) => {
                    res_or += 1 << i;
                }
                (Val::Zero, _) => {
                    res_and += 1 << i;
                }
                (Val::X, true) => {
                    res_or += 1 << i;
                    x_count += 1;
                }
                (Val::X, false) => {
                    x_count += 1;
                }
            }
        }
        self.permutation += 1;
        Some((res_and, res_or))
    }
}

impl From<&String> for Mask {
    fn from(input: &String) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let mut mask = [Val::Zero; 36];
        let mut num_x = 0;
        for i in 0..36 {
            mask[i] = match chars[i] {
                '1' => Val::One,
                '0' => Val::Zero,
                'X' => {
                    num_x += 1;
                    Val::X
                }
                _ => unreachable!(),
            }
        }

        Mask {
            mask,
            permutation: 0,
            max_x: 2u64.pow(num_x),
        }
    }
}

#[aoc_generator(day14)]
pub fn generate(input: &str) -> Vec<Parsed> {
    input
        .lines()
        .map(|line| line.parse().expect("could not parse line"))
        .collect()
}

#[aoc(day14, part1)]
pub fn part1(input: &[Parsed]) -> u64 {
    let mut memory = HashMap::new();
    let mut and_mask: u64 = 0;
    let mut or_mask: u64 = 0;
    for i in input {
        match i {
            Parsed::Mask(val) => {
                and_mask =
                    u64::from_str_radix(&val.replace("X", "1"), 2).expect("could not get val");
                or_mask =
                    u64::from_str_radix(&val.replace("X", "0"), 2).expect("could not get val");
            }
            Parsed::Addr(addr, val) => {
                memory.insert(addr, *val & and_mask | or_mask);
            }
        }
    }
    memory.iter().map(|(_key, val)| *val).sum()
}

#[aoc(day14, part2)]
pub fn part2(input: &[Parsed]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut curr_mask: Option<Mask> = None;
    for i in input {
        match i {
            Parsed::Mask(val) => curr_mask = Some(val.into()),
            Parsed::Addr(addr, val) => {
                if let Some(mut mask) = curr_mask {
                    while let Some((and_mask, or_mask)) = mask.next() {
                        memory.insert((*addr & and_mask) | or_mask, *val);
                    }
                }
            }
        }
    }
    memory.iter().map(|(_, val)| *val).sum::<u64>()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test() {
        let test = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";

        let gen = generate(&test);
        assert_eq!(165, part1(&gen));
    }

    #[test]
    pub fn test2() {
        let test = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mem[42] = 0
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
";

        let gen = generate(&test);
        assert_eq!(208, part2(&gen));
    }
}
