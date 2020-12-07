use pest::Parser;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
#[derive(Parser)]
#[grammar = "luggagerules.pest"]
pub struct LuggageRuleParser;

#[derive(Debug)]
pub struct LuggageRule {
    pub color: u64,
    pub contents: Option<Vec<Content>>,
}

#[derive(Debug)]
pub struct Content {
    pub color: u64,
    pub amount: u32,
}

pub fn hash_str(input: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

#[aoc_generator(day7)]
pub fn generate(input: &str) -> HashMap<u64, LuggageRule> {
    let parsed = LuggageRuleParser::parse(Rule::file, input).expect("could not parse! input");

    let mut all_rules = HashMap::new();
    for pair in parsed {
        for file_inner_pair in pair.into_inner() {
            let line = match file_inner_pair.as_rule() {
                Rule::line => {
                    let mut color = 0;
                    let mut contents: Vec<Content> = Vec::new();
                    for line_inner_pair in file_inner_pair.into_inner() {
                        match line_inner_pair.as_rule() {
                            Rule::bag => {
                                color = hash_str(line_inner_pair.as_str());
                            }
                            Rule::content => {
                                for content_outer_pair in line_inner_pair.into_inner() {
                                    match content_outer_pair.as_rule() {
                                        Rule::with_content => {
                                            let mut bag_amount = 0;
                                            let mut content_color = 0;
                                            for content_inner_pair in
                                                content_outer_pair.into_inner()
                                            {
                                                match content_inner_pair.as_rule() {
                                                    Rule::amount => {
                                                        bag_amount = content_inner_pair
                                                            .as_str()
                                                            .parse::<u32>()
                                                            .expect("could not parse amount");
                                                    }
                                                    Rule::bag => {
                                                        content_color =
                                                            hash_str(content_inner_pair.as_str());
                                                    }
                                                    _ => unreachable!(),
                                                }
                                            }
                                            let c = Content {
                                                color: content_color,
                                                amount: bag_amount,
                                            };
                                            contents.push(c);
                                        }
                                        Rule::no_content => {}
                                        _ => unreachable!(),
                                    };
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    let cont = if contents.is_empty() {
                        None
                    } else {
                        Some(contents)
                    };
                    Some(LuggageRule {
                        color,
                        contents: cont,
                    })
                }
                Rule::EOI => None,
                _ => panic!(format!("found {:?}", file_inner_pair.as_rule())),
            };
            if let Some(line) = line {
                all_rules.insert(line.color, line);
            }
        }
    }

    all_rules
}

lazy_static! {
    static ref SHINY: u64 = hash_str("shiny gold");
}

#[aoc(day7, part1)]
pub fn part1(all_rules: &HashMap<u64, LuggageRule>) -> usize {
    let mut contains = HashMap::new();
    for (color, _) in all_rules {
        does_contain_gold(color, all_rules, &mut contains);
    }
    contains.into_iter().filter(|(_, found)| *found).count()
}

pub fn does_contain_gold(
    color: &u64,
    all_rules: &HashMap<u64, LuggageRule>,
    checked: &mut HashMap<u64, bool>,
) -> bool {
    if let Some(has_checked) = checked.get(color) {
        return *has_checked;
    } else {
        let does_contain_gold = all_rules
            .get(color)
            .map(|rule| {
                rule.contents
                    .as_ref()
                    .map(|o_content| {
                        o_content.iter().any(|content| {
                            SHINY.eq(&content.color)
                                || does_contain_gold(&content.color, all_rules, checked)
                        })
                    })
                    .unwrap_or(false)
            })
            .expect(&format!("{} not in map!", color));
        checked.insert(*color, does_contain_gold);
        return does_contain_gold;
    }
}

#[aoc(day7, part2)]
pub fn part2(all_rules: &HashMap<u64, LuggageRule>) -> u32 {
    let mut num_bags: HashMap<u64, u32> = HashMap::new();
    calc_num_bags(&hash_str("shiny gold"), all_rules, &mut num_bags)
}

pub fn calc_num_bags(
    color: &u64,
    all_rules: &HashMap<u64, LuggageRule>,
    num_bags: &mut HashMap<u64, u32>,
) -> u32 {
    if let Some(baggies) = num_bags.get(color) {
        return *baggies;
    } else {
        let baggies = all_rules
            .get(color)
            .map(|rule| {
                rule.contents
                    .as_ref()
                    .map(|content| {
                        content
                            .iter()
                            .map(|c| c.amount * (calc_num_bags(&c.color, all_rules, num_bags) + 1))
                            .sum()
                    })
                    .unwrap_or(0)
            })
            .expect("color not found!");
        num_bags.insert(*color, baggies);
        return baggies;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn test_example() {
        let example = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";
        let generated = super::generate(&example);
        let res_1 = super::part1(&generated);
        let res_2 = super::part2(&generated);
        assert_eq!(4, res_1);
        assert_eq!(32, res_2);
    }
}
