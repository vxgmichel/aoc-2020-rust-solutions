use std::collections::HashSet;
use std::io::{self, BufRead};

type Cube = HashSet<(usize, usize, usize)>;
type HyperCube = HashSet<(usize, usize, usize, usize)>;

fn part1(xs: &[Vec<u8>], cycles: usize) -> usize {
    // Cube dimensions
    let rows = xs.len();
    let cols = xs[0].len();
    let depth = 1;

    // Initialize the cube
    let mut cube = HashSet::new();
    for (i, row) in xs.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == b'#' {
                cube.insert((i + cycles, j + cycles, cycles));
            }
        }
    }

    // Loop over cycles
    for i in 1..cycles + 1 {
        cube = next_cube(
            &cube,
            cycles - i,
            cycles + rows + i,
            cycles + cols + i,
            cycles + depth + i,
        );
    }

    // Count
    cube.len()
}

fn next_cube(cube: &Cube, start: usize, xstop: usize, ystop: usize, zstop: usize) -> Cube {
    (start..xstop)
        .flat_map(|x| (start..ystop).flat_map(move |y| (start..zstop).map(move |z| (x, y, z))))
        .filter(|&(x, y, z)| next_cell(cube, x, y, z))
        .collect()
}

fn next_cell(cube: &Cube, x: usize, y: usize, z: usize) -> bool {
    let c = (x.saturating_sub(1)..=x + 1)
        .flat_map(move |i| {
            (y.saturating_sub(1)..=y + 1)
                .flat_map(move |j| (z.saturating_sub(1)..=z + 1).map(move |k| (i, j, k)))
        })
        .filter(|&t| t != (x, y, z))
        .filter(|t| cube.contains(&t))
        .count();
    match (cube.contains(&(x, y, z)), c) {
        (true, 2..=3) => true,
        (true, _) => false,
        (false, 3) => true,
        (false, _) => false,
    }
}

fn part2(xs: &[Vec<u8>], cycles: usize) -> usize {
    // Cube dimensions
    let rows = xs.len();
    let cols = xs[0].len();
    let depth = 1;

    // Initialize the cube
    let mut cube = HashSet::new();
    for (i, row) in xs.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == b'#' {
                cube.insert((i + cycles, j + cycles, cycles, cycles));
            }
        }
    }

    // Loop over cycles
    for i in 1..cycles + 1 {
        cube = next_hypercube(
            &cube,
            cycles - i,
            cycles + rows + i,
            cycles + cols + i,
            cycles + depth + i,
        );
    }

    // Count
    cube.len()
}

fn next_hypercube(
    cube: &HyperCube,
    start: usize,
    xstop: usize,
    ystop: usize,
    zstop: usize,
) -> HyperCube {
    (start..xstop)
        .flat_map(|x| {
            (start..ystop).flat_map(move |y| {
                (start..zstop).flat_map(move |z| (start..zstop).map(move |h| (x, y, z, h)))
            })
        })
        .filter(|&(x, y, z, h)| next_hypercell(cube, x, y, z, h))
        .collect()
}

fn next_hypercell(cube: &HyperCube, x: usize, y: usize, z: usize, h: usize) -> bool {
    let count = (x.saturating_sub(1)..=x + 1)
        .flat_map(move |i| {
            (y.saturating_sub(1)..=y + 1).flat_map(move |j| {
                (z.saturating_sub(1)..=z + 1)
                    .flat_map(move |k| (h.saturating_sub(1)..=h + 1).map(move |l| (i, j, k, l)))
            })
        })
        .filter(|&t| t != (x, y, z, h))
        .filter(|t| cube.contains(&t))
        .count();
    match (cube.contains(&(x, y, z, h)), count) {
        (true, 2..=3) => true,
        (true, _) => false,
        (false, 3) => true,
        (false, _) => false,
    }
}

fn main() {
    let xs: Vec<_> = io::stdin()
        .lock()
        .lines()
        .filter_map(|x| Some(x.ok()?.as_bytes().to_vec()))
        .collect();
    let result = part1(&xs, 6);
    println!("Part 1: {}", result);
    let result = part2(&xs, 6);
    println!("Part 2: {}", result);
}
