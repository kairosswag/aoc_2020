use parse_display::{Display, FromStr};
use vector2d::Vector2D;

type Vec2 = Vector2D<i32>;

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy, Eq)]
#[display("{}{0}")]
pub enum Instruction {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

pub fn step_rotate_cw(vec: &Vec2, deg: i32) -> Vec2 {
    match deg {
        90 => Vec2::new(-vec.y, vec.x),
        180 => Vec2::new(-vec.x, -vec.y),
        270 => Vec2::new(vec.y, -vec.x),
        n=> panic!("unsupported movement, tried {} degrees", n),
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum Direction {
    E,
    S,
    W,
    N,
}

impl Direction {
    pub fn as_vec(self) -> Vec2 {
        match self {
            Direction::E => Vec2::new(0, 1),
            Direction::S => Vec2::new(-1, 0),
            Direction::W => Vec2::new(0, -1),
            Direction::N => Vec2::new(1, 0),
        }
    }
}

impl From<Vec2> for Direction {

    fn from(value: Vec2) -> Self {
        match (value.x, value.y) {
            (0, 1) => Direction::E,
            (-1, 0) => Direction::S,
            (0, -1) => Direction::W,
            (1, 0) => Direction::N,
            errval => panic!("Values not normalized (was: {:?})", errval),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub struct Ship {
    pos: Vec2,
    facing: Direction, // always normalized!
}

impl Ship {
    pub fn move_ship(&mut self, instr: &Instruction) {
        match instr {
            Instruction::N(val) => self.pos += Direction::N.as_vec() * *val,
            Instruction::S(val) => self.pos += Direction::S.as_vec() * *val,
            Instruction::E(val) => self.pos += Direction::E.as_vec() * *val,
            Instruction::W(val) => self.pos += Direction::W.as_vec() * *val,
            Instruction::L(deg) => self.facing = step_rotate_cw(&self.facing.as_vec(), 360 - deg).into(),
            Instruction::R(deg) => self.facing = step_rotate_cw(&self.facing.as_vec(), *deg).into(),
            Instruction::F(val) => self.pos += self.facing.as_vec() * *val,
        }
    }

    pub fn manhattan_origin(&self) -> i32 {
        self.pos.x.abs() + self.pos.y.abs()
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub struct GuidedShip {
    pos: Vec2,
    waypoint: Vec2,
}

impl GuidedShip {
    pub fn follow_instruction(&mut self, instr: &Instruction) {
        match instr {
            Instruction::N(val) => self.waypoint.x += val,
            Instruction::S(val) => self.waypoint.x -= val,
            Instruction::E(val) => self.waypoint.y += val,
            Instruction::W(val) => self.waypoint.y -= val,
            Instruction::L(deg) => self.waypoint = step_rotate_cw(&self.waypoint, 360 - deg),
            Instruction::R(deg) => self.waypoint = step_rotate_cw(&self.waypoint, *deg),
            Instruction::F(val) => self.pos += self.waypoint * *val,
        }
    }

    pub fn manhattan_origin(&self) -> i32 {
        self.pos.x.abs() + self.pos.y.abs()
    }
}

#[aoc_generator(day12)]
pub fn generate(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| l.parse().expect(&format!("Could not parse line: {}", l)))
        .collect()
}

#[aoc(day12, part1)]
pub fn part1(instructions: &[Instruction]) -> i32 {
    let ship = Ship {
        pos: Vec2::new(0, 0),
        facing: Direction::E,
    };
    let res = instructions.iter().fold(ship, |mut ship, instr| {
        ship.move_ship(instr);
        ship
    });

    res.manhattan_origin()
}

#[aoc(day12, part2)]
pub fn part2(instructions: &[Instruction]) -> i32 {
    let ship = GuidedShip {
        pos: Vec2::new(0, 0),
        waypoint: Vec2::new(1, 10),
    };
    let res = instructions.iter().fold(ship, |mut ship, instr| {
        ship.follow_instruction(instr);
        ship
    });

    res.manhattan_origin()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_example() {
        let input = 
        "F10
N3
F7
R90
F11";
        let gen = generate(&input);
        let res_ship = part1(&gen);
        assert_eq!(res_ship, 25);
        let res_ship_2 = part2(&gen);
        assert_eq!(res_ship_2, 286);
    }

    #[test]
    pub fn test_dir() {
        use Direction::*;

        assert_eq!(S, step_rotate_cw(&E.as_vec(), 90).into());
        assert_eq!(W, step_rotate_cw(&E.as_vec(), 180).into());
        assert_eq!(N, step_rotate_cw(&E.as_vec(), 270).into());

        
        assert_eq!(W, step_rotate_cw(&S.as_vec(), 90).into());
        assert_eq!(N, step_rotate_cw(&S.as_vec(), 180).into());
        assert_eq!(E, step_rotate_cw(&S.as_vec(), 270).into());

        
        assert_eq!(N, step_rotate_cw(&W.as_vec(), 90).into());
        assert_eq!(E, step_rotate_cw(&W.as_vec(), 180).into());
        assert_eq!(S, step_rotate_cw(&W.as_vec(), 270).into());
    }
}