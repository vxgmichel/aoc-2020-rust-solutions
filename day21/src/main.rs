use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

type Food = (HashSet<String>, HashSet<String>);
type Mapping = HashMap<String, HashSet<String>>;
type Reduced = HashMap<String, String>;

fn parse_line(line: &str) -> Option<Food> {
    let sep = " (contains ";
    let index = line.find(sep)?;
    let s1: HashSet<String> = line
        .get(0..index)?
        .split(' ')
        .map(|x| x.to_string())
        .collect();
    let s2: HashSet<String> = line
        .get(index + sep.len()..line.len() - 1)?
        .split(", ")
        .map(|x| x.to_string())
        .collect();
    Some((s1, s2))
}

fn build_mapping(xs: &[Food]) -> Mapping {
    let mut mapping: Mapping = HashMap::new();
    for (ys, zs) in xs {
        for z in zs {
            let current = mapping.entry(z.clone()).or_insert_with(|| ys.clone());
            *current = ys.intersection(current).cloned().collect();
        }
    }
    mapping
}

fn part1(xs: &[Food], mapping: &Mapping) -> usize {
    let suspects: HashSet<String> = mapping.values().flatten().cloned().collect();
    xs.iter()
        .flat_map(|(ys, _)| ys)
        .filter(|&x| !suspects.contains(x))
        .count()
}

fn reduce_mapping(mapping: &Mapping) -> Reduced {
    let mut result: Reduced = HashMap::new();
    let mut mapping = mapping.clone();
    while let Some((key, values)) = mapping.iter().find(|(_, values)| values.len() == 1) {
        let key = key.to_string();
        let value = values.iter().next().unwrap().to_string();
        mapping.remove(&key);
        for values in mapping.values_mut() {
            values.remove(&value);
        }
        result.insert(key, value);
    }
    result
}

fn part2(reduced: &Reduced) -> String {
    let mut vec: Vec<_> = reduced.iter().collect();
    vec.sort_by_key(|(key, _)| key.to_string());
    vec.iter()
        .map(|&(_, value)| value)
        .cloned()
        .collect::<Vec<_>>()
        .join(",")
}

fn main() {
    let xs: Vec<_> = io::stdin()
        .lock()
        .lines()
        .filter_map(|x| parse_line(x.ok()?.as_ref()))
        .collect();
    let mapping = build_mapping(&xs);
    let result = part1(&xs, &mapping);
    println!("Part 1: {}", result);
    let reduced = reduce_mapping(&mapping);
    let result = part2(&reduced);
    println!("Part 2: {}", result);
}
