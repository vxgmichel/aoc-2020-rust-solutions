use std::cmp::Ordering;
use std::io::{self, BufRead};

fn check(xs: &[usize], n: usize) -> bool {
    xs.iter()
        .flat_map(|x| xs.iter().map(move |y| (x, y)))
        .any(|(x, y)| x != y && x + y == n)
}

fn part1(xs: &[usize]) -> usize {
    xs.windows(26)
        .filter(|x| !check(&x[..25], x[25]))
        .map(|x| x[25])
        .next()
        .unwrap()
}

fn part2(xs: &[usize], t: usize) -> usize {
    let ys = rec2(xs, t, 0, 0, 0);
    ys.iter().min().unwrap() + ys.iter().max().unwrap()
}

fn rec2(xs: &[usize], t: usize, a: usize, b: usize, s: usize) -> &[usize] {
    match t.cmp(&s) {
        Ordering::Equal => &xs[a..b],
        Ordering::Greater => rec2(xs, t, a, b + 1, s + xs[b]),
        Ordering::Less => rec2(xs, t, a + 1, b, s - xs[a]),
    }
}

fn main() {
    let vec: Vec<usize> = io::stdin()
        .lock()
        .lines()
        .filter_map(|x| x.ok()?.parse().ok())
        .collect();
    let result = part1(&vec);
    println!("Part 1: {}", result);
    let result = part2(&vec, result);
    println!("Part 2: {}", result);
}
