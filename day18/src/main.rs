use std::io::{self, BufRead};

mod part1;
mod part2;

fn run_part1(xs: &[String]) -> usize {
    xs.iter().map(|x| part1::evaluate(x.as_bytes())).sum()
}

fn run_part2(xs: &[String]) -> usize {
    xs.iter().map(|x| part2::evaluate(x.as_bytes())).sum()
}

fn main() {
    let xs: Vec<String> = io::stdin().lock().lines().filter_map(|x| x.ok()).collect();
    let result = run_part1(&xs);
    println!("Part 1: {}", result);
    let result = run_part2(&xs);
    println!("Part 2: {}", result);
}
