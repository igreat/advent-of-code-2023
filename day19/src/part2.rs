use std::collections::HashMap;

pub fn run(input: &str) -> usize {
    let rules = parse_input(input);
    let ranges = [(1, 4000), (1, 4000), (1, 4000), (1, 4000)];
    let start_rule = &rules["in"];
    accepted_combinations(&rules, start_rule, &mut ranges.to_vec())
}

fn accepted_combinations(
    all_rules: &HashMap<String, Vec<Rule>>,
    rules: &[Rule],
    ranges: &mut [(usize, usize)],
) -> usize {
    if rules.is_empty() {
        return 0;
    }

    let rule = &rules[0];
    let mut ranges_other = vec![];

    if let (Some(category), Some(condition)) = (&rule.category, &rule.condition) {
        let range_idx = match category {
            Category::X => 0,
            Category::M => 1,
            Category::A => 2,
            Category::S => 3,
        };

        match condition {
            Condition::GreaterThan(n) => {
                if ranges[range_idx].1 > *n {
                    ranges_other = ranges.to_vec();
                    ranges[range_idx].0 = *n + 1;
                    ranges_other[range_idx].1 = *n;
                }
            }
            Condition::LessThan(n) => {
                if ranges[range_idx].0 < *n {
                    ranges_other = ranges.to_vec();
                    ranges[range_idx].1 = *n - 1;
                    ranges_other[range_idx].0 = *n;
                }
            }
        }
    }

    if ranges_other.is_empty() {
        match &rule.then {
            Then::Accept => ranges.iter().map(|(min, max)| max - min + 1).product(),
            Then::Reject => 0,
            Then::Goto(id) => {
                let rules = all_rules.get(id).unwrap();
                accepted_combinations(all_rules, rules, ranges)
            }
        }
    } else {
        let mut count = 0;

        // first, move the ranges to the next id
        match &rule.then {
            Then::Accept => {
                count += ranges
                    .iter()
                    .map(|(min, max)| max - min + 1)
                    .product::<usize>();
            }
            Then::Reject => {}
            Then::Goto(id) => {
                let rules = all_rules.get(id).unwrap();
                count += accepted_combinations(all_rules, rules, ranges);
            }
        }

        // then, move the ranges_clone to the next rule
        let next_rules = &rules[1..];
        count += accepted_combinations(all_rules, next_rules, &mut ranges_other);

        count
    }
}

#[derive(Debug, PartialEq)]
enum Condition {
    GreaterThan(usize),
    LessThan(usize),
}

#[derive(Debug, PartialEq)]
enum Then {
    Accept,
    Reject,
    Goto(String),
}

#[derive(Debug, PartialEq)]
enum Category {
    X = 0,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq)]
struct Rule {
    category: Option<Category>,
    condition: Option<Condition>,
    then: Then,
}

#[derive(Debug, PartialEq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

fn parse_input(input: &str) -> HashMap<String, Vec<Rule>> {
    let (rules, _) = input.split_once("\n\n").unwrap();
    rules
        .lines()
        .map(|line| {
            let mut parts = line.splitn(2, '{');
            let id = parts.next().unwrap();
            let rules = parts
                .next()
                .unwrap()
                .trim_end_matches('}')
                .split(',')
                .map(|rules| parse_rule(rules))
                .collect();
            (id.to_string(), rules)
        })
        .collect()
}

fn parse_rule(rules: &str) -> Rule {
    // first check if ':' is present
    if let Some((condition, then)) = rules.split_once(':') {
        // if it is, then we have a condition
        let category = match condition.as_bytes()[0] {
            b'x' => Category::X,
            b'm' => Category::M,
            b'a' => Category::A,
            b's' => Category::S,
            _ => panic!("Invalid category"),
        };
        let condition = match condition.as_bytes()[1] {
            b'>' => Condition::GreaterThan(condition[2..].parse().unwrap()),
            b'<' => Condition::LessThan(condition[2..].parse().unwrap()),
            _ => panic!("Invalid condition"),
        };
        let then = match then {
            "A" => Then::Accept,
            "R" => Then::Reject,
            _ => Then::Goto(then.to_string()),
        };
        Rule {
            category: Some(category),
            condition: Some(condition),
            then,
        }
    } else {
        // otherwise, we have a goto
        Rule {
            category: None,
            condition: None,
            then: match rules {
                "A" => Then::Accept,
                "R" => Then::Reject,
                _ => Then::Goto(rules.to_string()),
            },
        }
    }
}
