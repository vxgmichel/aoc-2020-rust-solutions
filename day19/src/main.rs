use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
enum Rule {
    Number(usize),
    Char(char),
    Union(Vec<Rule>),
    Concat(Vec<Rule>),
}

type Rules = HashMap<usize, Rule>;

fn parse_rule_value(value: &str) -> Option<Rule> {
    if value.starts_with('"') {
        let arg = value.chars().nth(1)?;
        Some(Rule::Char(arg))
    } else if value.contains(" | ") {
        let arg = value.split(" | ").filter_map(parse_rule_value).collect();
        Some(Rule::Union(arg))
    } else if value.contains(' ') {
        let arg = value.split(' ').filter_map(parse_rule_value).collect();
        Some(Rule::Concat(arg))
    } else {
        let arg = value.parse().ok()?;
        Some(Rule::Number(arg))
    }
}

fn parse_rule(line: &str) -> Option<(usize, Rule)> {
    let mut it = line.split(": ");
    let id = it.next()?.parse().ok()?;
    let value = it.next()?.to_string();
    assert!(it.next().is_none());
    Some((id, parse_rule_value(&value)?))
}

fn parse(lines: &[String]) -> Option<(Rules, Vec<String>)> {
    let mut it = lines.split(|x| x.is_empty());
    let rules = it.next()?.iter().filter_map(|x| parse_rule(x)).collect();
    let data = it.next()?.to_vec();
    Some((rules, data))
}

fn solve(rules: &Rules, data: &[String]) -> usize {
    let rule = Rule::Number(0);
    data.iter()
        .filter(|x| do_match(rules, x, &rule).contains(&""))
        .count()
}

fn do_match<'a>(rules: &Rules, x: &'a str, r: &Rule) -> Vec<&'a str> {
    match r {
        Rule::Number(n) => do_match(rules, x, rules.get(&n).unwrap()),
        Rule::Char(c) => {
            if x.starts_with(*c) {
                vec![&x[c.len_utf8()..]]
            } else {
                vec![]
            }
        }
        Rule::Union(rs) => match &rs[..] {
            [] => vec![],
            [r, rs @ ..] => do_match(rules, x, r)
                .iter()
                .chain(do_match(rules, x, &Rule::Union(rs.to_vec())).iter())
                .cloned()
                .collect(),
        },
        Rule::Concat(rs) => match &rs[..] {
            [] => vec![x],
            [r, rs @ ..] => do_match(rules, x, r)
                .iter()
                .flat_map(|y| do_match(rules, y, &Rule::Concat(rs.to_vec())))
                .collect(),
        },
    }
}

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().filter_map(|x| x.ok()).collect();
    let (mut rules, data) = parse(&lines).unwrap();
    let result = solve(&rules, &data);
    println!("Part 1: {}", result);
    rules
        .entry(8)
        .and_modify(|e| *e = parse_rule_value("42 | 42 8").unwrap());
    rules
        .entry(11)
        .and_modify(|e| *e = parse_rule_value("42 31 | 42 11 31").unwrap());
    let result = solve(&rules, &data);
    println!("Part 2: {:?}", result);
}
