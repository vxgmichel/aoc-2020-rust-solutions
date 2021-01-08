use std::collections::HashMap;
use std::io::{self, BufRead};
use std::iter::repeat;

fn solve(xs: &[usize], target: usize) -> usize {
    let mut next = 0;
    let mut map = HashMap::new();
    for (i, x) in xs
        .iter()
        .map(Some)
        .chain(repeat(None))
        .enumerate()
        .take(target - 1)
    {
        map.entry(*x.unwrap_or(&next))
            .and_modify(|e| {
                next = i - *e;
                *e = i
            })
            .or_insert_with(|| {
                next = 0;
                i
            });
    }
    next
}

fn main() {
    let stdin = io::stdin();
    let line: String = stdin.lock().lines().next().unwrap().unwrap();
    let xs: Vec<_> = line
        .split(",")
        .map(|x| x.parse())
        .collect::<Result<_, _>>()
        .unwrap();
    let result = solve(&xs, 2020);
    println!("Part 1: {}", result);
    let result = solve(&xs, 30000000);
    println!("Part 2: {}", result);
}
