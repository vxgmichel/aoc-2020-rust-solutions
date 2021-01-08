use std::io::{self, BufRead};

fn main() {
    let mut vec: Vec<_> = io::stdin()
        .lock()
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| {
            x.replace("F", "0")
                .replace("B", "1")
                .replace("L", "0")
                .replace("R", "1")
        })
        .filter_map(|x| u32::from_str_radix(&x, 2).ok())
        .collect();
    vec.sort_unstable();
    println!("Part 1: {}", vec.last().unwrap());
    let result = vec
        .iter()
        .zip(vec[1..].iter())
        .filter(|(&x, &y)| x + 2 == y)
        .map(|(x, _)| x + 1)
        .next()
        .unwrap();
    println!("Part 2: {}", result)
}
