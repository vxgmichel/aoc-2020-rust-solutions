use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn part1(lines: &[String]) -> usize {
    lines.join("").chars().collect::<HashSet<char>>().len()
}

fn part2(lines: &[String]) -> usize {
    let mut map = lines[0].chars().map(|x| (x, 1)).collect::<HashMap<_, _>>();
    lines[1..].iter().flat_map(|x| x.chars()).for_each(|x| {
        map.entry(x).and_modify(|e| *e += 1);
    });
    map.iter().filter(|(_, &v)| v == lines.len()).count()
}

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().filter_map(|x| x.ok()).collect();
    let result: usize = lines.split(|x| x == "").map(part1).sum();
    println!("Part 1: {}", result);
    let result: usize = lines.split(|x| x == "").map(part2).sum();
    println!("Part 2: {}", result)
}
