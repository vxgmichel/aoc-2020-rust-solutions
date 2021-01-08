use std::io::{self, BufRead};

fn count(vec: &[String], a: usize, b: usize) -> usize {
    // Loop over the vector
    vec.iter()
        // Use b as a step to ignore some lines
        .step_by(b)
        // Use enumerate to get the index
        .enumerate()
        // Keep the encounters with a tree
        .filter(|(i, x)| x.as_bytes()[i * a % x.len()] == b'#')
        // Count those encounters
        .count()
}

fn solve(slopes: &[(usize, usize)], vec: &[String]) -> usize {
    // The result is the product of the result of each slope
    slopes.iter().map(|&(a, b)| count(vec, a, b)).product()
}

fn main() {
    // The difference between part 1 and part 2 are the slopes to use
    let slopes_1 = vec![(3, 1)];
    let slopes_2 = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    // Read the input data as vector of string from stdin
    let vec: Vec<String> = io::stdin().lock().lines().filter_map(|x| x.ok()).collect();
    // Display the result
    println!("Part 1: {}", solve(&slopes_1, &vec));
    println!("Part 2: {}", solve(&slopes_2, &vec));
}
