use std::collections::HashSet;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy, Eq)]
#[display("{} {val}")]
#[allow(non_camel_case_types)]
pub enum Instruction {
    nop { val: i32 },
    acc { val: i32 },
    jmp { val: i32 },
}

#[aoc_generator(day8)]
pub fn generate(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.parse().expect("could not parse"))
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &[Instruction]) -> i32 {
    let mut instruction_ptr = 0;
    let mut accum = 0;
    let mut visited = HashSet::new();
    while !visited.contains(&instruction_ptr) {
        visited.insert(instruction_ptr);

        use Instruction::*;
        match input[instruction_ptr] {
            nop { val: _ } => instruction_ptr += 1,
            acc { val } => {
                accum += val;
                instruction_ptr += 1;
            }
            jmp { val } => instruction_ptr = ((instruction_ptr as i32) + val) as usize,
        }
    }
    accum
}

#[aoc(day8, part2)]
pub fn part2(input: &[Instruction]) -> i32 {
    let mut curr_replace = None; //0 is acc anyway so no changes
    let mut already_replaced = HashSet::new();
    loop {
        match try_path_replacement(input, &curr_replace) {
            Ok(accum) => return accum,
            Err(backtrack) => curr_replace = next_backtrack(input, &backtrack, &already_replaced),
        }
        already_replaced.insert(curr_replace.unwrap());
    }
}

pub fn next_backtrack(
    input: &[Instruction],
    backtrack: &[usize],
    already_replaced: &HashSet<usize>,
) -> Option<usize> {
    for ptr in backtrack.iter().rev() {
        if !already_replaced.contains(ptr) {
            match input[*ptr] {
                Instruction::nop { val: _ } | Instruction::jmp { val: _ } => return Some(*ptr),
                Instruction::acc { val: _ } => {}
            }
        }
    }
    panic!("no viable alternative found!");
}

pub fn try_path_replacement(
    input: &[Instruction],
    replaced: &Option<usize>,
) -> Result<i32, Vec<usize>> {
    let mut instruction_ptr = 0;
    let mut accum = 0;
    let mut backtrack = Vec::new();
    while instruction_ptr < input.len() && !backtrack.contains(&instruction_ptr) {
        backtrack.push(instruction_ptr);

        let to_replace = replaced.map_or(false, |r| r == instruction_ptr);

        use Instruction::*;
        match (input[instruction_ptr], to_replace) {
            (nop { val: _ }, false) | (jmp { val: _ }, true) => instruction_ptr += 1,
            (acc { val }, _) => {
                accum += val;
                instruction_ptr += 1;
            }
            (jmp { val }, false) | (nop { val }, true) => {
                instruction_ptr = ((instruction_ptr as i32) + val) as usize
            }
        }
    }
    if instruction_ptr >= input.len() {
        Ok(accum)
    } else {
        Err(backtrack)
    }
}
