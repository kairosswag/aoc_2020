use parse_display::{Display, FromStr};
use regex::Regex;

lazy_static! {
    static ref HAIR_MATCH: Regex = Regex::new(r"^#[a-z0-9]{6}$").unwrap();
    static ref PID_MATCH: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{}:{content}")]
#[allow(non_camel_case_types)]
pub enum PassportField {
    byr { content: String },
    iyr { content: String },
    eyr { content: String },
    hgt { content: String },
    hcl { content: String },
    ecl { content: String },
    pid { content: String },
    cid { content: String },
}

impl PassportField {
    fn validate(&self) -> bool {
        match self {
            PassportField::byr { content } => content
                .parse::<u32>()
                .map_or(false, |val| val >= 1920 && val <= 2002),
            PassportField::iyr { content } => content
                .parse::<u32>()
                .map_or(false, |val| val >= 2010 && val <= 2020),
            PassportField::eyr { content } => content
                .parse::<u32>()
                .map_or(false, |val| val >= 2020 && val <= 2030),
            PassportField::hgt { content } => match (
                content.get(0..content.len() - 2),
                content.get(content.len() - 2..),
            ) {
                (Some(h_cm), Some("cm")) => {
                    h_cm.parse::<u32>().map_or(false, |h| h >= 150 && h <= 193)
                }
                (Some(h_in), Some("in")) => {
                    h_in.parse::<u32>().map_or(false, |h| h >= 59 && h <= 76)
                }
                (_, _) => false,
            },
            PassportField::hcl { content } => HAIR_MATCH.is_match(content),
            PassportField::ecl { content } => {
                content == "amb"
                    || content == "blu"
                    || content == "brn"
                    || content == "gry"
                    || content == "grn"
                    || content == "hzl"
                    || content == "oth"
            }
            PassportField::pid { content } => PID_MATCH.is_match(content),
            PassportField::cid { content: _ } => true,
        }
    }
}

#[aoc_generator(day4)]
pub fn generate(input: &str) -> Vec<Vec<PassportField>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse().expect("Could not parse field!"))
                .collect::<Vec<PassportField>>()
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Vec<PassportField>]) -> usize {
    let mut counter = 0;
    let mut found = (false, false, false, false, false, false, false, false);
    for field_line in input {
        if field_line.is_empty() {
            if let (true, true, true, true, true, true, true, _) = found {
                counter += 1;
            }

            found = (false, false, false, false, false, false, false, false);
        } else {
            for field in field_line {
                match field {
                    PassportField::byr { content: _ } => found.0 = true,
                    PassportField::iyr { content: _ } => found.1 = true,
                    PassportField::eyr { content: _ } => found.2 = true,
                    PassportField::hgt { content: _ } => found.3 = true,
                    PassportField::hcl { content: _ } => found.4 = true,
                    PassportField::ecl { content: _ } => found.5 = true,
                    PassportField::pid { content: _ } => found.6 = true,
                    PassportField::cid { content: _ } => found.7 = true,
                }
            }
        }
    }

    if let (true, true, true, true, true, true, true, _) = found {
        counter += 1;
    }

    counter
}

#[aoc(day4, part2)]
pub fn part2(input: &[Vec<PassportField>]) -> usize {
    let mut counter = 0;
    let mut found = (false, false, false, false, false, false, false, false);
    for field_line in input {
        if field_line.is_empty() {
            if let (true, true, true, true, true, true, true, _) = found {
                counter += 1;
            }

            found = (false, false, false, false, false, false, false, false);
        } else {
            for field in field_line {
                match (field, field.validate()) {
                    (PassportField::byr { content: _ }, true) => found.0 = true,
                    (PassportField::iyr { content: _ }, true) => found.1 = true,
                    (PassportField::eyr { content: _ }, true) => found.2 = true,
                    (PassportField::hgt { content: _ }, true) => found.3 = true,
                    (PassportField::hcl { content: _ }, true) => found.4 = true,
                    (PassportField::ecl { content: _ }, true) => found.5 = true,
                    (PassportField::pid { content: _ }, true) => found.6 = true,
                    (PassportField::cid { content: _ }, true) => found.7 = true,
                    _ => {}
                }
            }
        }
    }

    if let (true, true, true, true, true, true, true, _) = found {
        counter += 1;
    }

    counter
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn test() {
        let text = "test whitespace sth"
            .split_ascii_whitespace()
            .collect::<Vec<&str>>();
        let no_text = "".split_ascii_whitespace().collect::<Vec<&str>>();
        assert_eq!(text, vec!["test", "whitespace", "sth"]);
        let empty: Vec<&str> = Vec::new();
        assert_eq!(no_text, empty);
    }

    #[test]
    pub fn test_example() {
        let input = "
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
            ";

        let generated = super::generate(input);
        assert_eq!(2, super::part1(&generated));
    }

    #[test]
    pub fn test_invalid() {
        let input = "
        eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
        ";

        let generated = super::generate(input);
        assert_eq!(0, super::part2(&generated));
    }

    #[test]
    pub fn test_valid() {
        let input = "
        pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        ";

        let generated = super::generate(input);
        assert_eq!(4, super::part2(&generated));
    }
}
