use std::collections::HashMap;
use std::io::{self, BufRead};

fn parse_value(x: &str) -> Option<(usize, String)> {
    let mut it = x.split(' ');
    let count = it.next()?;
    let adjective = it.next()?;
    let color = it.next()?;
    let bags = it.next()?;
    if it.next().is_some() {
        return None;
    }
    assert!(["bag", "bags"].contains(&bags));
    Some((
        count.parse::<usize>().ok()?,
        format!("{} {}", adjective, color),
    ))
}

fn parse_line(x: String) -> Option<(String, Vec<(usize, String)>)> {
    let mut it = x[..x.len() - 1].split(" contain ");
    let key_string = it.next()?;
    let value_string = it.next()?;
    if it.next().is_some() {
        return None;
    }
    let key = key_string.rsplitn(2, ' ').nth(1)?.to_string();
    if value_string == "no more bags" {
        return Some((key, vec![]));
    }
    let values: Vec<_> = value_string
        .split(", ")
        .filter_map(|x| parse_value(x))
        .collect();
    Some((key, values))
}

fn rec1(graph: &HashMap<String, Vec<(usize, String)>>, target: &str, name: &str) -> bool {
    if name == target {
        true
    } else {
        graph
            .get(name)
            .unwrap()
            .iter()
            .any(|(_, child)| rec1(graph, target, child))
    }
}

fn rec2(graph: &HashMap<String, Vec<(usize, String)>>, name: &str) -> usize {
    1 + graph
        .get(name)
        .unwrap()
        .iter()
        .map(|(x, child)| x * rec2(graph, child))
        .sum::<usize>()
}

fn main() {
    // Read and parse input data as a graph
    let graph: HashMap<_, _> = io::stdin()
        .lock()
        .lines()
        .filter_map(|x| x.ok())
        .filter_map(parse_line)
        .collect();
    // Use recursive solvers
    let result1 = graph
        .iter()
        .filter(|(key, _)| rec1(&graph, &"shiny gold", key))
        .count()
        - 1;
    let result2 = rec2(&graph, &"shiny gold") - 1;
    // Display the result
    println!("Part 1: {:?}", result1);
    println!("Part 2: {:?}", result2);
}
