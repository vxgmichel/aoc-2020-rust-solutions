use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
use std::iter::once;

type Rule = [(usize, usize); 2];
type Rules = HashMap<String, Rule>;
type Ticket = Vec<usize>;

fn parse_rule(line: &str) -> Option<(String, Rule)> {
    let mut it1 = line.split(": ");
    let name = it1.next()?.to_string();
    let mut it2 = it1.next()?.split(" or ");
    let mut itr1 = it2.next()?.split('-');
    let mut itr2 = it2.next()?.split('-');
    let a = itr1.next()?.parse().ok()?;
    let b = itr1.next()?.parse().ok()?;
    let c = itr2.next()?.parse().ok()?;
    let d = itr2.next()?.parse().ok()?;
    Some((name, [(a, b), (c, d)]))
}

fn parse_ticket(s: &str) -> Ticket {
    s.split(',').filter_map(|x| x.parse().ok()).collect()
}

fn parse(lines: &[String]) -> Option<(Rules, Ticket, Vec<Ticket>)> {
    let mut it = lines.split(|x| x.is_empty());
    let rules = it.next()?.iter().filter_map(|x| parse_rule(x)).collect();
    let ticket = parse_ticket(it.next()?.get(1)?);
    let tickets = it
        .next()?
        .get(1..)?
        .iter()
        .map(|x| parse_ticket(&x))
        .collect();
    Some((rules, ticket, tickets))
}

fn is_valid(value: usize, rules: &Rules) -> bool {
    rules
        .values()
        .flatten()
        .any(|&(a, b)| a <= value && value <= b)
}

fn part1(rules: &Rules, tickets: &[Ticket]) -> usize {
    tickets
        .iter()
        .flatten()
        .filter(|&x| !is_valid(*x, rules))
        .sum()
}

fn part2(rules: &Rules, ticket: Ticket, tickets: &[Ticket]) -> usize {
    let valid_tickets: Vec<&Ticket> = tickets
        .iter()
        .filter(|&x| x.iter().all(|&y| is_valid(y, rules)))
        .chain(once(&ticket))
        .collect();

    // Loop over column index
    let mut matching = (0..ticket.len())
        // Produce a vector of (index, name) tuples
        .map(|i| {
            // Keep the index because of the upcoming sort
            (
                i,
                // Loop over rules
                rules
                    .iter()
                    // Filter rules that are inconsistent with the column i
                    .filter(|(_, &[(a, b), (c, d)])| {
                        // Loop over valid tickets
                        valid_tickets
                            .iter()
                            // Check the ith value of the ticket against the current rule
                            .all(|x| (a <= x[i] && x[i] <= b) || (c <= x[i] && x[i] <= d))
                    })
                    // Keep the name
                    .map(|(name, _)| name)
                    // Collect as a vector of names
                    .collect::<Vec<_>>(),
            )
        })
        // Collect as a vector of vector
        .collect::<Vec<_>>();

    matching.sort_by_key(|(_, x)| x.len());

    let mut seen = HashSet::<&&String>::new();
    let mut solved = HashMap::new();
    for (i, x) in matching.iter() {
        let mut current = x.iter().collect::<HashSet<_>>();
        current.retain(|x| !seen.contains(x));
        seen.extend(current.iter());
        assert!(current.len() == 1);
        solved.insert(current.iter().cloned().next().unwrap(), i);
    }

    solved
        .iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, &i)| ticket[*i])
        .product()
}

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().filter_map(|x| x.ok()).collect();
    let (rules, ticket, tickets) = parse(&lines).unwrap();
    let result = part1(&rules, &tickets);
    println!("Part 1: {}", result);
    let result = part2(&rules, ticket, &tickets);
    println!("Part 2: {}", result);
}
