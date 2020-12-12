use std::{convert::TryInto, slice::Iter};

use parse_display::Display;

#[derive(PartialEq, Debug, Clone, Copy, Eq, Display)]
pub enum Position {
    #[display(".")]
    Floor,
    #[display("L")]
    Empty,
    #[display("#")]
    Occupied,
}

impl Position {
    pub fn from_1<'a>(in_char: &'a u8) -> Iter<'a, Position> {
        match *in_char {
            46 => [Position::Floor].iter(),
            76 => [Position::Empty].iter(),
            10 => [Position::Floor, Position::Floor].iter(),
            chr => panic!("Could not parse {}", chr),
        }
    }

    pub fn from_2(in_char: &u8) -> Option<Position> {
        match *in_char {
            46 => Some(Position::Floor),
            76 => Some(Position::Empty),
            10 => None,
            chr => panic!("Could not parse {}", chr),
        }
    }

    pub fn next_from_3x3(
        a1: Position,
        a2: Position,
        a3: Position,
        b1: Position,
        b2: Position,
        b3: Position,
        c1: Position,
        c2: Position,
        c3: Position,
    ) -> (Position, bool) {
        use Position::*;
        let count = [a1, a2, a3, b1, b3, c1, c2, c3]
            .iter()
            .map(Position::count_occupied)
            .sum();
        match (b2, count) {
            (Floor, _) => (Floor, false),
            (Empty, 0) => (Occupied, true),
            (Empty, _) => (Empty, false),
            (Occupied, x) if x >= 4 => (Empty, true),
            (Occupied, _) => (Occupied, false),
        }
    }

    pub fn count_occupied(&self) -> u8 {
        match self {
            Position::Occupied => 1,
            _ => 0,
        }
    }
}

pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

const LINE_LEN_1: usize = 99;
const LINE_LEN_2: usize = 97;
const LINE_LEN_2_I: i32 = 97;

#[aoc_generator(day11, part1)]
pub fn generate(input: &[u8]) -> Vec<Position> {
    let mut start = vec![Position::Floor; LINE_LEN_1 + 1]; // line with floors plus one extra floor
    let mut end = vec![Position::Floor; LINE_LEN_1 + 1];
    let mut mtrx = input
        .iter()
        .flat_map(|x| Position::from_1(x))
        .map(|pos| *pos)
        .collect();

    start.append(&mut mtrx);
    start.append(&mut end);

    start
}

#[aoc_generator(day11, part2)]
pub fn generate2(input: &[u8]) -> Vec<Position> {
    input.iter().flat_map(|x| Position::from_2(x)).collect()
}

#[aoc(day11, part1)]
pub fn part1(matrix: &[Position]) -> u32 {
    let len = matrix.len();
    let mut res_mtrx_a = vec![Position::Floor; len];
    let mut res_mtrx_b = vec![Position::Floor; len];

    let mut found = conv(&matrix, &mut res_mtrx_a, len);

    loop {
        if !found {
            return res_mtrx_a.iter().map(|p| p.count_occupied() as u32).sum();
        }
        found = conv(&res_mtrx_a, &mut res_mtrx_b, len);
        if !found {
            return res_mtrx_b.iter().map(|p| p.count_occupied() as u32).sum();
        }
        found = conv(&res_mtrx_b, &mut res_mtrx_a, len);
    }
}

pub fn conv(matrix: &[Position], res_matrix: &mut [Position], len: usize) -> bool {
    let mut has_changed = false;
    for x in 1..((len / LINE_LEN_1) - 1) {
        for y in 1..(LINE_LEN_1 - 1) {
            let a1 = (x - 1) * LINE_LEN_1 + (y - 1);
            let a2 = a1 + 1;
            let a3 = a2 + 1;
            let b1 = a1 + LINE_LEN_1;
            let b2 = b1 + 1;
            let b3 = b2 + 1;
            let c1 = b1 + LINE_LEN_1;
            let c2 = c1 + 1;
            let c3 = c2 + 1;
            let (res_pos, changed) = Position::next_from_3x3(
                matrix[a1], matrix[a2], matrix[a3], matrix[b1], matrix[b2], matrix[b3], matrix[c1],
                matrix[c2], matrix[c3],
            );
            res_matrix[b2] = res_pos;
            has_changed |= changed;
        }
    }
    has_changed
}

#[aoc(day11, part2)]
pub fn part2(matrix: &[Position]) -> u32 {
    let len = matrix.len();
    let mut res_mtrx_a = vec![Position::Floor; len];
    let mut res_mtrx_b = vec![Position::Floor; len];

    let mut found = conv2(&matrix, &mut res_mtrx_a);

    loop {
        if !found {
            return res_mtrx_a.iter().map(|p| p.count_occupied() as u32).sum();
        }
        found = conv2(&res_mtrx_a, &mut res_mtrx_b);
        if !found {
            return res_mtrx_b.iter().map(|p| p.count_occupied() as u32).sum();
        }
        found = conv2(&res_mtrx_b, &mut res_mtrx_a);

    }
}

pub fn conv2(matrix: &[Position], res_matrix: &mut [Position]) -> bool {
    let mut has_changed = false;
    for x in 0..matrix.len() / LINE_LEN_2 {
        for y in 0..LINE_LEN_2 {
            let pos = x * LINE_LEN_2 + y;
            res_matrix[pos] = match matrix[pos] {
                Position::Floor => Position::Floor,
                Position::Empty => {
                    if !loop_directions(x, y, matrix).any(|x| x) {
                        has_changed = true;
                        Position::Occupied
                    } else {
                        Position::Empty
                    }
                }
                Position::Occupied => {
                    if let None =
                        loop_directions(x, y, matrix)
                            .map(|b| b as u32)
                            .try_fold(0, |acc, found| {
                                let next = acc + found;
                                if next < 5 {
                                    Some(next)
                                } else {
                                    None
                                }
                            })
                    {
                        has_changed = true;
                        Position::Empty
                    } else {
                        Position::Occupied
                    }
                }
            }
        }
    }
    has_changed
}

pub fn loop_directions<'a>(
    start_pos_x: usize,
    start_pos_y: usize,
    matrix: &'a [Position],
) -> impl Iterator<Item = bool> + 'a {
    [
        Direction::N,
        Direction::NE,
        Direction::E,
        Direction::SE,
        Direction::S,
        Direction::SW,
        Direction::W,
        Direction::NW,
    ]
    .iter()
    .map(move |dir| dir_occupied(start_pos_x, start_pos_y, matrix, dir))
}

pub fn dir_occupied(
    start_pos_x: usize,
    start_pos_y: usize,
    matrix: &[Position],
    dir: &Direction,
) -> bool {
    let mut step = 1;
    while let Some(idx) = step_dir(
        start_pos_x.try_into().unwrap(),
        start_pos_y.try_into().unwrap(),
        step,
        matrix.len().try_into().unwrap(),
        &dir,
    ) {
        match matrix[idx] {
            Position::Occupied => return true,
            Position::Empty => return false,
            Position::Floor => step += 1,
        }
    }
    false
}

pub fn step_dir(
    start_pos_x: i32,
    start_pos_y: i32,
    step: i32,
    len: i32,
    dir: &Direction,
) -> Option<usize> {
    let (x, y) = match dir {
        Direction::N => (start_pos_x - step, start_pos_y),
        Direction::NE => (start_pos_x - step, start_pos_y + step),
        Direction::E => (start_pos_x, start_pos_y + step),
        Direction::SE => (start_pos_x + step, start_pos_y + step),
        Direction::S => (start_pos_x + step, start_pos_y),
        Direction::SW => (start_pos_x + step, start_pos_y - step),
        Direction::W => (start_pos_x, start_pos_y - step),
        Direction::NW => (start_pos_x - step, start_pos_y - step),
    };

    let idx = x * LINE_LEN_2_I + y;
    if x >= 0 && y >= 0 && x < (len / LINE_LEN_2_I) && y < LINE_LEN_2_I && idx < len {
        Some(idx as usize)
    } else {
        None
    }
}
