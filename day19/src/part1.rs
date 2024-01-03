use std::collections::HashMap;

pub fn run(input: &str) -> usize {
    let (rules, parts) = parse_input(input);
    let mut count = 0;
    let mut rule_id;
    for part in parts {
        rule_id = "in".to_string();
        'part: loop {
            let rules = rules.get(&rule_id).unwrap();
            for rule in rules {
                let mut is_valid = true;
                if rule.category.is_some() {
                    let category = rule.category.as_ref().unwrap();
                    let condition = rule.condition.as_ref().unwrap();

                    let value = match category {
                        Category::X => part.x,
                        Category::M => part.m,
                        Category::A => part.a,
                        Category::S => part.s,
                    };

                    is_valid = match condition {
                        Condition::GreaterThan(n) => value > *n,
                        Condition::LessThan(n) => value < *n,
                    };
                };

                if !is_valid {
                    continue;
                }

                match &rule.then {
                    Then::Accept => {
                        count += part.x + part.m + part.a + part.s;
                        break 'part;
                    }
                    Then::Reject => break 'part,
                    Then::Goto(id) => {
                        rule_id = id.to_string();
                        continue 'part;
                    }
                }
            }
        }
    }
    count
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
    X,
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

fn parse_input(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
    let (rules, ratings) = input.split_once("\n\n").unwrap();
    let rules = rules
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
        .collect();

    let parts: Vec<Part> = ratings
        .lines()
        .map(|line| {
            let line = &line[1..line.len() - 1];
            let ratings: Vec<usize> = line
                .split(',')
                .map(|rating| rating[2..].parse().unwrap())
                .collect();

            Part {
                x: ratings[0],
                m: ratings[1],
                a: ratings[2],
                s: ratings[3],
            }
        })
        .collect();

    (rules, parts)
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
