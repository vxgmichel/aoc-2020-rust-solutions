use std::io::{self, BufRead};

fn run(xs: &[usize], steps: usize) -> Vec<usize> {
    let size = xs.len();

    // Prepare next structure
    let mut next = vec![0; size + 1];
    let mut x1 = xs[size - 1];
    for &x2 in xs {
        next[x1] = x2;
        x1 = x2;
    }

    // Loop over steps
    let mut current = xs[0];
    for _ in 0..steps {
        // Get next 3 elements
        let a = next[current];
        let b = next[a];
        let c = next[b];
        let d = next[c];

        // Find destination
        let mut dest = current;
        while dest == current || dest == a || dest == b || dest == c {
            dest = if dest == 1 { size } else { dest - 1 };
        }
        let after_dest = next[dest];

        // Update
        next[c] = after_dest;
        next[current] = d;
        next[dest] = a;
        current = d;
    }

    let mut result = vec![1];
    let mut current = next[1];
    while current != 1 {
        result.push(current);
        current = next[current];
    }
    result
}

fn part1(xs: &[usize]) -> String {
    let result = run(xs, 10);
    result.iter().skip(1).map(|x| x.to_string()).collect()
}

fn part2(xs: &[usize]) -> usize {
    let xs: Vec<usize> = xs.iter().copied().chain(xs.len() + 1..=1000000).collect();
    let result = run(&xs, 10000000);
    result.iter().skip(1).take(2).product()
}

fn main() {
    let xs: Vec<usize> = io::stdin()
        .lock()
        .lines()
        .filter_map(|x| x.ok())
        .next()
        .unwrap()
        .chars()
        .filter_map(|x| x.to_digit(10).map(|x| x as usize))
        .collect();
    let result = part1(&xs);
    println!("Part 1: {}", result);
    let result = part2(&xs);
    println!("Part 2: {}", result);
}
