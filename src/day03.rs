const TREE: char = '#';
const ROW_LEN: usize = 31;


#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .map(|(line_no, line)| line.as_bytes()[(line_no * 3) % ROW_LEN])
        .filter(|&t| t as char == TREE)
        .count()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let res = input
        .lines()
        .enumerate()
        .map(|(line_no, line)| (line_no, line.as_bytes()))
        .map(|(line_no, line)| {
            (
                calc_tree_hit(line_no, line, 1, 1),
                calc_tree_hit(line_no, line, 3, 1),
                calc_tree_hit(line_no, line, 5, 1),
                calc_tree_hit(line_no, line, 7, 1),
                calc_tree_hit(line_no, line, 1, 2),
            )
        })
        .fold((0, 0, 0, 0, 0), |acc, val| {
            (
                acc.0 + val.0,
                acc.1 + val.1,
                acc.2 + val.2,
                acc.3 + val.3,
                acc.4 + val.4,
            )
        });
    res.0 * res.1 * res.2 * res.3 * res.4
}

pub fn calc_tree_hit(line_no: usize, line: &[u8], right: usize, down: usize) -> u32 {
    if line_no % down == 0 && // might skip lines
        line[((line_no /  down) * right) % ROW_LEN] as char == TREE
    {
        1
    } else {
        0
    }
}
