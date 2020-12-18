use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Eq)]
pub enum Token {
    #[display("+")]
    OpAdd,
    #[display("*")]
    OpMul,
    #[display("(")]
    ParOpen,
    #[display(")")]
    ParClose,
    #[display(" ")]
    Space,
    #[display("{0}")]
    Number(u64),
    #[display("nopewhat")]
    None,
}

impl Token {
    pub fn apply(&self, arg_a: u64, arg_b: u64) -> u64 {
        let res = match self {
            Token::OpAdd => arg_a + arg_b,
            Token::OpMul => arg_a * arg_b,
            Token::None => arg_b,
            _ => unreachable!(),
        };
        res
    }
}

///```
/// assert_eq!(2, advent_of_code_2020::day18::part1(&"2"));
/// assert_eq!(26, advent_of_code_2020::day18::part1(&"2 * 3 + (4 * 5)"));
/// assert_eq!(13632, advent_of_code_2020::day18::part1(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
///```
#[aoc(day18, part1)]
pub fn part1(input: &str) -> u64 {
    input.lines().map(|l| parse_and_solve_term(l)).sum()
}

pub fn parse_and_solve_term(line: &str) -> u64 {
    let mut acc = 0;
    let mut curr_op = Token::None;
    let mut out_par_start_idx = 0;
    let mut par_layer = 0;
    for i in 0..line.len() {
        match (line[i..i + 1].parse().expect("nah"), par_layer) {
            (Token::OpAdd, 0) => curr_op = Token::OpAdd,
            (Token::OpMul, 0) => curr_op = Token::OpMul,
            (Token::ParOpen, 0) => {
                out_par_start_idx = i;
                par_layer += 1;
            }
            (Token::ParOpen, _) => {
                par_layer += 1;
            }
            (Token::ParClose, 1) => {
                par_layer -= 1;
                acc = curr_op.apply(acc, parse_and_solve_term(&line[out_par_start_idx + 1..i]));
            }
            (Token::ParClose, _) => {
                par_layer -= 1;
            }
            (Token::Number(num), 0) => acc = curr_op.apply(acc, num),
            (Token::Space, _) => {}
            (_, a) if a > 0 => {}
            _ => {}
        }
    }
    acc
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> u64 {
    input.lines().map(|l| parse_and_solve_term_spec_rec(l)).sum()
}

// take the first highest layer operator and split at that
pub fn parse_and_solve_term_spec_rec(line: &str) -> u64 {
    let mut first_mul_idx = None;
    let mut first_add_idx = None;
    let mut layer = 0;
    let mut last_number = (0, 0);
    for idx in 0..line.len() {
        match line[idx..idx + 1].parse().expect("nah") {
            Token::ParOpen => layer += 1,
            Token::ParClose => layer -= 1,
            Token::OpMul if first_mul_idx.map_or(true, |(_, lyr)| lyr > layer) => {
                first_mul_idx = Some((idx, layer))
            }
            Token::OpAdd if first_add_idx.map_or(true, |(_, lyr)| lyr > layer) => {
                first_add_idx = Some((idx, layer))
            }
            Token::Number(val) => last_number = (val, last_number.1 + 1),
            Token::None | Token::Space | Token::OpMul | Token::OpAdd => {}
        }
    }
    match (first_mul_idx, first_add_idx) {
        (Some((mul_idx, _)), None) => Token::OpMul.apply(
            parse_and_solve_term_spec_rec(&line[..mul_idx - 1]),
            parse_and_solve_term_spec_rec(&line[mul_idx + 1..]),
        ),
        (None, Some((add_idx, _))) => Token::OpAdd.apply(
            parse_and_solve_term_spec_rec(&line[..add_idx - 1]),
            parse_and_solve_term_spec_rec(&line[add_idx + 1..]),
        ),
        (Some((mul_idx, mul_lyr)), Some((_, add_lyr))) if mul_lyr <= add_lyr => Token::OpMul
            .apply(
                parse_and_solve_term_spec_rec(&line[..mul_idx - 1]),
                parse_and_solve_term_spec_rec(&line[mul_idx + 1..]),
            ),
        (Some((_, mul_lyr)), Some((add_idx, add_lyr))) if mul_lyr > add_lyr => Token::OpAdd
            .apply(
                parse_and_solve_term_spec_rec(&line[..add_idx - 1]),
                parse_and_solve_term_spec_rec(&line[add_idx + 1..]),
            ),
        (None, None) => {
            if last_number.1 != 1 {
                println!("first_mul_x {:?}", first_mul_idx);
                println!("first_add_x {:?}", first_add_idx);
                println!("line {}", line);
                panic!("unexpected number: ist not last of kind")
            };
            last_number.0
        },
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test1() {
        assert_eq!(20, part1(&"(4 * 5)"));
    }
}
