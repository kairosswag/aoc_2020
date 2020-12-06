use std::collections::HashMap;

use itertools::Itertools;



#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let mut groups = Vec::new();
    let mut group = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            groups.push(group);
            group = Vec::new();
        } else {
            for c in line.chars() {
                group.push(c);
            }
        }
    }
    if !group.is_empty() {
        groups.push(group);
    }

    groups.iter().map(|group| group.iter().unique().count()).sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let mut groups = Vec::new();
    let mut group = HashMap::new();
    let mut num_entries = 0;
    for line in input.lines() {
        if line.is_empty() {
            groups.push((group, num_entries));
            group = HashMap::new();
            num_entries = 0;
        } else {
            num_entries += 1;
            for c in line.chars() {
                let counter = group.entry(c).or_insert( 0);
                *counter += 1;
            }
        }
    }
    if !group.is_empty() {
        groups.push((group, num_entries));
    }

    groups.iter().map(|(group, ne)| group.iter().filter(|(_key, count)| count == &ne).count()).sum()

}