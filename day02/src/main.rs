use std::io::{self, BufRead};

fn part1(vec: &[(usize, usize, u8, Vec<u8>)]) -> usize {
    vec.iter()
        .map(|(x, y, c, p)| (x, y, p.iter().filter(|&z| z == c).count()))
        .filter(|&(&x, &y, c)| x <= c && c <= y)
        .count()
}

fn part2(vec: &[(usize, usize, u8, Vec<u8>)]) -> usize {
    vec.iter()
        .filter(|(x, y, c, p)| (p[x - 1] == *c) ^ (p[y - 1] == *c))
        .count()
}

fn parse_line(line: &str) -> (usize, usize, u8, Vec<u8>) {
    let mut iter = line.split(' ');
    let mut xy = iter.next().unwrap().split('-');
    let c = iter.next().unwrap().bytes().next().unwrap();
    let p = iter.next().unwrap().as_bytes().to_vec();
    let x = xy.next().unwrap().parse().unwrap();
    let y = xy.next().unwrap().parse().unwrap();
    (x, y, c, p)
}

fn main() {
    let vec: Vec<(usize, usize, u8, Vec<u8>)> = io::stdin()
        .lock()
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| parse_line(&x))
        .collect();
    println!("Part 1: {}", part1(&vec));
    println!("Part 2: {}", part2(&vec));
}
