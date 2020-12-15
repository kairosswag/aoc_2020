use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn generate(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|c| c.parse::<usize>().expect("could not parse"))
        .collect()
}

#[aoc(day15, part1)]
pub fn part1(starting_numbers: &[usize]) -> usize {
    calc2(2020, starting_numbers)
}

#[aoc(day15, part2)]
pub fn part2(starting_numbers: &[usize]) -> usize {
    calc2(30000000, starting_numbers)
}

pub fn calc(max: usize, starting_numbers: &[usize]) -> usize {
    let mut spoken = HashMap::new();
    for i in 1..=starting_numbers.len() {
        spoken.insert(starting_numbers[i - 1], i);
    }
    let mut recent_spoken = starting_numbers[starting_numbers.len() - 1];
    (starting_numbers.len()..max).fold(recent_spoken, |recent_spoken, i| {
        i - spoken.insert(recent_spoken, i).unwrap_or(i)
    })
}

pub fn calc2(max: usize, starting_numbers: &[usize]) -> usize {
    let mut spoken = vec![0; max];
    for i in 1..=starting_numbers.len() {
        spoken[starting_numbers[i - 1]] = i;
    }
    let mut recent_spoken = starting_numbers[starting_numbers.len() - 1];
    (starting_numbers.len()..max).fold(recent_spoken, |recent_spoken, i| {
        let mut prev = spoken[recent_spoken];
        spoken[recent_spoken] = i;
        if prev == 0 {
            prev = i;
        }
        i - prev
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test() {
        // let input = "0,3,6";
        let input1 = "1,3,2";
        let input2 = "2,1,3";
        let input3 = "1,2,3";
        let input4 = "2,3,1";
        // assert_eq!(1, part1(&generate(&input)));
        assert_eq!(1, part1(&generate(&input1)));
        assert_eq!(10, part1(&generate(&input2)));
        assert_eq!(27, part1(&generate(&input3)));
        assert_eq!(78, part1(&generate(&input4)));
    }
}
