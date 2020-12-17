use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

#[derive(Debug)]
pub enum Cube {
    Active,
    Inactive,
}
impl Cube {
    pub fn is_active(&self) -> bool {
        match self {
            Cube::Active => true,
            Cube::Inactive => false,
        }
    }
    pub fn get_str(&self) -> &'static str {
        match self {
            Cube::Active => "#",
            Cube::Inactive => ".",
        }
    }
}

impl From<char> for Cube {
    fn from(c: char) -> Self {
        match c {
            '#' => Cube::Active,
            '.' => Cube::Inactive,
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day17, part1)]
pub fn generate(input: &str) -> Rc<RefCell<HashMap<(i32, i32, i32), Cube>>> {
    Rc::new(RefCell::new(
        input
            .lines()
            .enumerate()
            .flat_map(|(l_idx, line)| {
                line.chars().enumerate().map(move |(c_idx, c_val)| {
                    ((l_idx as i32, c_idx as i32, 0 as i32), c_val.into())
                })
            })
            .collect(),
    ))
}

#[aoc(day17, part1)]
pub fn part1(coordinates: &Rc<RefCell<HashMap<(i32, i32, i32), Cube>>>) -> usize {
    let mut curr_coordinates = coordinates.clone();
    for _i in 0..6 {
        let inserting_coordinates = Rc::new(RefCell::new(HashMap::new()));
        let (min, max) = calculate_min_max(&curr_coordinates.borrow());
        for x in min.0 - 1..=max.0 + 1 {
            for y in min.1 - 1..=max.1 + 1 {
                for z in min.2 - 1..=max.2 + 1 {
                    let neighbors = calc_neighbors(x, y, z, &curr_coordinates.borrow());
                    inserting_coordinates
                        .borrow_mut()
                        .insert((x, y, z), neighbors);
                }
            }
        }

        curr_coordinates = inserting_coordinates.clone();
    }
    let res = curr_coordinates
        .borrow()
        .values()
        .filter(|c| c.is_active())
        .count();
    res
}

fn calc_neighbors(
    x: i32,
    y: i32,
    z: i32,
    curr_coordinates: &HashMap<(i32, i32, i32), Cube>,
) -> Cube {
    let mut active_neighbors = 0;
    for n_x in x - 1..=x + 1 {
        for n_y in y - 1..=y + 1 {
            for n_z in z - 1..=z + 1 {
                if n_x != x || n_y != y || n_z != z {
                    match curr_coordinates.get(&(n_x, n_y, n_z)) {
                        Some(Cube::Active) => active_neighbors += 1,
                        _ => {}
                    }
                    if active_neighbors > 3 {
                        return Cube::Inactive;
                    }
                }
            }
        }
    }

    match (active_neighbors, curr_coordinates.get(&(x, y, z))) {
        (3, _) => Cube::Active,
        (2, Some(Cube::Active)) => Cube::Active,
        _ => Cube::Inactive,
    }
}

pub fn calculate_min_max(
    coordinates: &HashMap<(i32, i32, i32), Cube>,
) -> ((i32, i32, i32), (i32, i32, i32)) {
    coordinates.keys().fold(
        (
            (std::i32::MAX, std::i32::MAX, std::i32::MAX),
            (std::i32::MIN, std::i32::MIN, std::i32::MIN),
        ),
        |acc, key| {
            (
                (
                    acc.0 .0.min(key.0),
                    acc.0 .1.min(key.1),
                    acc.0 .2.min(key.2),
                ),
                (
                    acc.1 .0.max(key.0),
                    acc.1 .1.max(key.1),
                    acc.1 .2.max(key.2),
                ),
            )
        },
    )
}

#[aoc_generator(day17, part2)]
pub fn generate_hyper(input: &str) -> Rc<RefCell<HashMap<(i32, i32, i32, i32), Cube>>> {
    Rc::new(RefCell::new(
        input
            .lines()
            .enumerate()
            .flat_map(|(l_idx, line)| {
                line.chars().enumerate().map(move |(c_idx, c_val)| {
                    (
                        (l_idx as i32, c_idx as i32, 0 as i32, 0 as i32),
                        c_val.into(),
                    )
                })
            })
            .collect(),
    ))
}

#[aoc(day17, part2)]
pub fn part2(coordinates: &Rc<RefCell<HashMap<(i32, i32, i32, i32), Cube>>>) -> usize {
    let mut curr_coordinates = coordinates.clone();
    for _i in 0..6 {
        let inserting_coordinates = Rc::new(RefCell::new(HashMap::new()));
        let (min, max) = calculate_min_max_hyper(&curr_coordinates.borrow());
        for x in min.0 - 1..=max.0 + 1 {
            for y in min.1 - 1..=max.1 + 1 {
                for z in min.2 - 1..=max.2 + 1 {
                    for h in min.3 - 1..=max.3 + 1 {
                        let neighbors =
                            calc_neighbors_hyper(x, y, z, h, &curr_coordinates.borrow());
                        inserting_coordinates
                            .borrow_mut()
                            .insert((x, y, z, h), neighbors);
                    }
                }
            }
        }

        curr_coordinates = inserting_coordinates.clone();
    }
    let res = curr_coordinates
        .borrow()
        .values()
        .filter(|c| c.is_active())
        .count();
    res
}

fn calc_neighbors_hyper(
    x: i32,
    y: i32,
    z: i32,
    h: i32,
    curr_coordinates: &HashMap<(i32, i32, i32, i32), Cube>,
) -> Cube {
    let mut active_neighbors = 0;
    for n_x in x - 1..=x + 1 {
        for n_y in y - 1..=y + 1 {
            for n_z in z - 1..=z + 1 {
                for n_h in h - 1..=h + 1 {
                    if n_x != x || n_y != y || n_z != z || n_h != h {
                        match curr_coordinates.get(&(n_x, n_y, n_z, n_h)) {
                            Some(Cube::Active) => active_neighbors += 1,
                            _ => {}
                        }
                        if active_neighbors > 3 {
                            return Cube::Inactive;
                        }
                    }
                }
            }
        }
    }

    match (active_neighbors, curr_coordinates.get(&(x, y, z, h))) {
        (3, _) => Cube::Active,
        (2, Some(Cube::Active)) => Cube::Active,
        _ => Cube::Inactive,
    }
}

pub fn calculate_min_max_hyper(
    coordinates: &HashMap<(i32, i32, i32, i32), Cube>,
) -> ((i32, i32, i32, i32), (i32, i32, i32, i32)) {
    coordinates.keys().fold(
        (
            (std::i32::MAX, std::i32::MAX, std::i32::MAX, std::i32::MAX),
            (std::i32::MIN, std::i32::MIN, std::i32::MIN, std::i32::MIN),
        ),
        |acc, key| {
            (
                (
                    acc.0 .0.min(key.0),
                    acc.0 .1.min(key.1),
                    acc.0 .2.min(key.2),
                    acc.0 .3.min(key.3),
                ),
                (
                    acc.1 .0.max(key.0),
                    acc.1 .1.max(key.1),
                    acc.1 .2.max(key.2),
                    acc.1 .3.max(key.3),
                ),
            )
        },
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test1() {
        let input = ".#.
..#
###";

        assert_eq!(112, part1(&generate(input)));
        assert_eq!(848, part2(&generate_hyper(input)));
    }
}
