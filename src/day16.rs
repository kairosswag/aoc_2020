use std::{collections::HashSet, vec};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Eq)]
#[display(
    "{rule_name}: {first_range_start}-{first_range_end} or {second_range_start}-{second_range_end}"
)]
pub struct Rule {
    pub rule_name: String,
    pub first_range_start: u32,
    pub first_range_end: u32,
    pub second_range_start: u32,
    pub second_range_end: u32,
}

impl Rule {
    pub fn clears(&self, it: u32) -> bool {
        (it >= self.first_range_start && it <= self.first_range_end)
            || (it >= self.second_range_start && it <= self.second_range_end)
    }
}

#[derive(PartialEq, Debug, Clone, Eq)]
pub struct Note {
    pub rules: Vec<Rule>,
    pub my_ticket: Vec<u32>,
    pub nearby_tickets: Vec<Vec<u32>>,
}

enum Mode {
    Rules,
    MyTicket,
    NearbyTickets,
    Await,
}

#[aoc_generator(day16)]
pub fn generate(input: &str) -> Note {
    let mut mode = Mode::Rules;
    let mut fields = Vec::new();
    let mut my_ticket = Vec::new();
    let mut nearby_tickets = Vec::new();

    for line in input.lines() {
        match (&mode, line.is_empty()) {
            (Mode::Rules, false) => {
                fields.push(line.parse().expect("Could not parse field!"));
            }
            (Mode::MyTicket, false) => {
                my_ticket = parse_ticket_numbers(line);
            }
            (Mode::NearbyTickets, false) => {
                nearby_tickets.push(parse_ticket_numbers(line));
            }
            (Mode::MyTicket, true) | (Mode::Rules, true) => {
                mode = Mode::Await;
            }
            (Mode::Await, false) => {
                mode = match line {
                    "your ticket:" => Mode::MyTicket,
                    "nearby tickets:" => Mode::NearbyTickets,
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    Note {
        rules: fields,
        my_ticket,
        nearby_tickets,
    }
}

fn parse_ticket_numbers(line: &str) -> Vec<u32> {
    line.split(",")
        .map(|n| n.parse().expect("could not parse number"))
        .collect()
}

#[aoc(day16, part1)]
pub fn part1(input: &Note) -> u32 {
    let rules = &input.rules;
    input
        .nearby_tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .filter(|&val| !rules.iter().any(|rule| rule.clears(*val)))
                .sum::<u32>()
        })
        .sum()
}

#[aoc(day16, part2)]
pub fn part2(input: &Note) -> u64 {
    let num_rows = input.rules.len();
    let rules = &input.rules;
    let mut viable = vec![vec![true; num_rows]; num_rows];
    let valid_tickets: &Vec<&Vec<u32>> = &input
        .nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|&val| rules.iter().any(|rule| rule.clears(val)))
        })
        .collect();
    for ticket in valid_tickets {
        for val_idx in 0..num_rows {
            for rule_idx in 0..num_rows {
                viable[rule_idx][val_idx] &= rules[rule_idx].clears(ticket[val_idx]);
                // println!("class {} clears {}?: {}", rules[rule_idx].rule_name, ticket[val_idx], viable[rule_idx][val_idx]);
            }
        }
    }

    // for rule_idx in 0..num_rows {
    //     for val_idx in 0..num_rows {
    //         print!("{}", if viable[rule_idx][val_idx] { "🟩" } else { "🟥" });
    //     }
    //     println!(" {}", rules[rule_idx].rule_name);
    // }

    let mut found_rows = HashSet::new();
    let mut res: u64 = 1;
    'outer: while found_rows.len() != num_rows {
        // println!("rows: {}/{}", found_rows.len(), num_rows);
        for rul_idx in 0..num_rows {
            let rule = &viable[rul_idx];
            let count: Vec<(usize, &bool)> = rule
                .iter()
                .enumerate()
                .map(|(idx, v)| (idx +1, v))
                .filter(|(idx, v)| **v && !found_rows.contains(idx))
                .collect();

            // println!("rule idx {} found {}", rul_idx, count.len());
            if count.len() == 1 {
                found_rows.insert(count[0].0);
                match rules[rul_idx].rule_name.get(..9) {
                    Some("departure") => res *= input.my_ticket[count[0].0 - 1] as u64,
                    _ => {}
                }
                // println!("rule {} @ {}", rules[rul_idx], count[0].0);
                continue 'outer;
            }
        }
        unreachable!()
    }
    res
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test1() {
        let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

        assert_eq!(1, part2(&generate(input)));

    }
}
