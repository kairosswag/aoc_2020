use regex::Regex;
#[derive(Debug)]
pub struct Entry {
    pub lowest: u32,
    pub highest: u32,
    pub letter: char,
    pub password: String,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d*)-(\d*) (.): (.*)").unwrap();
}

impl From<&str> for Entry {

    fn from(line: &str) -> Self {
        let x = RE.captures_iter(line).next().expect("no match found!");
        Entry {
            lowest: x[1].parse::<u32>().expect("no u32 in first spot"),
            highest: x[2].parse::<u32>().expect("no u32 in second spot"),
            letter: x[3].chars().next().expect("no char"),
            password: x[4].into(),
        }
    }
}



#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Entry> {
    input.lines().map(|line| Entry::from(line)).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Entry]) -> u32 {
    input.iter()
         .map(|e| (e, e.password.chars().filter(|&c| c == e.letter).count() as u32))
        //  .map(|x| {println!("{:?}", x); x})
         .filter(|(e, c)| c >= &e.lowest && c <= &e.highest)
         .count() as u32
}

#[aoc(day2, part2)]
pub fn part2(input: &[Entry]) -> usize {
    input.iter()
         .map(|e| (e.letter, e.password.chars(), e.lowest, e.highest))
         .map(|(pw_char, password_chars, low_idx, high_idx)| (pw_char, {
             let mut skipped = password_chars.skip((low_idx - 1) as usize);
             let char_1 = skipped.next().expect("ERRNOVAL_1");
             let char_2 = skipped.skip((high_idx - low_idx - 1) as usize).next().expect("ERRNOVAL_2");
             (char_1, char_2)
            }))
         .filter(|(x, (f, s))| (x == f || x == s) && f != s)
         .count()
}