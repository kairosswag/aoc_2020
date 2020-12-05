
pub struct BoardingPass {
    pub board_id: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct BoardingCollector {
    pub accum: u32,
    pub min: u32,
    pub max: u32,
}

impl BoardingCollector {

    pub fn new() -> BoardingCollector {
        BoardingCollector { accum: 0, min: std::u32::MAX, max: 0}
    }

    pub fn from(pass: &BoardingPass) -> BoardingCollector {
        BoardingCollector { accum: pass.board_id, min: pass.board_id, max: pass.board_id }
    }

    pub fn fold(&self, other: u32) -> BoardingCollector {
        BoardingCollector { accum: self.accum + other, min: u32::min(self.min, other), max: u32::max(self.max, other)}
    }

}

impl From<&str> for BoardingPass {

    fn from(s: &str) -> Self {
        let mut board_id = 0;
        for (idx, char) in s.chars().enumerate() {
            match char {
                'B' | 'R' => board_id |= 1 << (9-idx), // partition is 7-idx << 3 due to *8, seating is 9-idx due to starting at 7 with len 3
                'L' | 'F' => {},
                u => panic!("unexpected char! {}", u),
            }
        }

        BoardingPass { board_id }
    }
}


#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<BoardingPass> {
    input.lines().map(|s| BoardingPass::from(s)).collect()
}

#[aoc(day5, part1)]
pub fn part1(passes: &[BoardingPass]) -> u32 {
    passes.iter().map(|pass|pass.board_id).max().expect("no max value")
}

#[aoc(day5, part2)]
pub fn part2(passes: &[BoardingPass]) -> u32 {
    let collect = passes.iter().map(|pass| pass.board_id).fold(BoardingCollector::new(), |accum, b| accum.fold(b));
    // add all numbers up to max
    let up_to_max = collect.max * (collect.max + 1) / 2;

    // add all numbers up to min - 1
    let up_to_min = (collect.min-1) * (collect.min) / 2;

    
    let expected = up_to_max - up_to_min;
    
    let missing = expected - collect.accum;

    missing
}

#[cfg(test)]
mod tests {
    use super::BoardingPass;


    #[test]
    pub fn test_passes() {
        let passes = vec![BoardingPass::from("BFFFBBFRRR"), BoardingPass::from("FFFBBBFRRR"), BoardingPass::from("BBFFBBFRLL")];
        assert_eq!(820, super::part1(&passes));
    }

}


