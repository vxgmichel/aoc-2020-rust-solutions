use std::cmp;
use std::collections::HashMap;
use std::io::{self, BufRead};

fn part1(xs: &[usize]) -> usize {
    let mut map = HashMap::new();
    for (x, y) in xs.iter().zip(xs[1..].iter()) {
        *map.entry(y - x).or_insert(0) += 1;
    }
    map.get(&1).unwrap() * map.get(&3).unwrap()
}

fn rec2(xs: &[usize], i: usize, cache: &mut HashMap<usize, u64>) -> u64 {
    if i == xs.len() - 1 {
        return 1;
    }
    if let Some(&x) = cache.get(&i) {
        return x;
    }
    let a = i + 1;
    let b = cmp::min(a + 3, xs.len());
    let result = (a..b)
        .filter(|&j| xs[j] - xs[i] <= 3)
        .map(|j| rec2(xs, j, cache))
        .sum();
    cache.insert(i, result);
    result
}

fn part2(xs: &[usize]) -> u64 {
    let mut cache = HashMap::new();
    rec2(xs, 0, &mut cache)
}

fn main() {
    let mut vec: Vec<usize> = io::stdin()
        .lock()
        .lines()
        .filter_map(|x| x.ok()?.parse().ok())
        .collect();
    vec.push(0);
    vec.sort_unstable();
    vec.push(vec.last().unwrap() + 3);
    let result = part1(&vec);
    println!("Part 1: {}", result);
    let result = part2(&vec);
    println!("Part 2: {}", result);
}

// fn part2(xs: &[usize]) -> u64 {
//     let mut fib = vec![1, 1, 2];
//     let mut result = 1;
//     let mut index = 0;

//     for (x, y) in xs.iter().zip(xs[1..].iter()) {
//         match y - x {
//             1 => {
//                 index += 1;
//                 if index == fib.len() {
//                     fib.push(fib[index - 1] + fib[index - 2] + fib[index - 3])
//                 }
//             }
//             3 => {
//                 result *= fib[index];
//                 index = 0;
//             }
//             _ => panic!(),
//         }
//     }
//     result
// }
