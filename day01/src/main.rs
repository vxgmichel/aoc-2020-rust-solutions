use std::io::{self, BufRead};

fn part1(vec: &[u32]) -> u32 {
    vec.iter()
        .enumerate()
        .flat_map(|(i, x)| vec[i + 1..].iter().map(move |y| (x, y)))
        .filter(|&(x, y)| x + y == 2020)
        .map(|(x, y)| x * y)
        .next()
        .unwrap()
}

fn part2(vec: &[u32]) -> u32 {
    vec.iter()
        .flat_map(|x| {
            vec.iter()
                .flat_map(move |y| vec.iter().map(move |z| (x, y, z)))
        })
        .filter(|&(x, y, z)| x + y + z == 2020)
        .map(|(x, y, z)| x * y * z)
        .next()
        .unwrap()
}

fn main() {
    let vec: Vec<u32> = io::stdin()
        .lock()
        .lines()
        .filter_map(|x| x.ok())
        .filter_map(|x| x.parse().ok())
        .collect();
    println!("Part 1: {}", part1(&vec));
    println!("Part 2: {}", part2(&vec));
}
