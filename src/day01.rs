#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().expect("line is no string")).collect()  
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    for x in 0..&input.len() - 1 {
        for y in (x+1)..input.len() {
            if input[x] + input[y] == 2020 {
                return input[x] * input[y];
            }
        }
    }
    panic!("no result found!")
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    for x in 0..&input.len() - 1 {
        for y in (x+1)..input.len() {
            for z in (y+1)..input.len() {
                if input[x] + input[y] + input[z] == 2020 {
                    return input[x] * input[y] * input[z];
                }
            }
        }
    }
    panic!("no result found!")
}