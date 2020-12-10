use itertools::Itertools;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| l.parse::<u32>().expect("line is no integer"))
        .sorted()
        .collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &[u32]) -> u32 {
    let acc = [0]
        .iter()
        .chain(input)
        .zip(input.iter())
        .map(|(a, b)| b.max(a) - b.min(a))
        .fold((0, 1), |(acc_1, acc_3), v| match v {
            1 => (acc_1 + 1, acc_3),
            3 => (acc_1, acc_3 + 1),
            x => {
                println!("eh?, {:?}, {}", (acc_1, acc_3), x);
                (acc_1, acc_3)
            }
        });
    acc.0 * acc.1
}

#[aoc(day10, part2)]
pub fn part2(input: &[u32]) -> u128 {
    // analysis from part1: there are no other distances besides 1 and 3

    // within groups all distances are 1 with grplen being number of consecutive ones:
    // 1 => all
    // 2 => all + none
    // 3 => l + r + lr + none
    // 4 => l + m + r + lm + mr + lr + none
    [0].iter()
        .chain(input)
        .zip(input.iter())
        .map(|(a, b)| b.max(a) - b.min(a))
        .group_by(|k| *k != 3)
        .into_iter()
        .filter_map(|(ones, group)| match ones {
            true => Some(group.count()),
            false => None,
        })
        // .filter(|count| *count > 2) // cannot permutate a group consisting of 2 numbers
        .map(|count| match count {
            1 => 1,
            2 => 2 as u128,
            3 => 4,
            4 => 7,
            x => panic!("{} not suported", x),
        })
        .fold(1 as u128, |accum, val| accum * val)
}
