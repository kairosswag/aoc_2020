#[aoc_generator(day9)]
pub fn generate(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|l| l.parse::<u64>().expect("could not parse line"))
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(numbers: &[u64]) -> u64 {
    'outer: for i in 25..numbers.len() {
        for j in i - 25..i {
            for k in j + 1..i {
                if numbers[j] + numbers[k] == numbers[i] {
                    continue 'outer;
                }
            }
        }
        return numbers[i];
    }
    panic!("no result found!");
}

#[aoc(day9, part2)]
pub fn part2(numbers: &[u64]) -> u64 {
    let res_1 = 675280050;
    for i in 0..numbers.len() {
        let mut sum = numbers[i];
        let mut min = numbers[i];
        let mut max = numbers[i];
        let mut offset = i;
        while sum < res_1 {
            offset += 1;
            sum += numbers[offset];
            min = min.min(numbers[offset]);
            max = max.max(numbers[offset]);
        }
        if sum == res_1 {
            return min + max;
        }
    }
    panic!("no result found!");
}
