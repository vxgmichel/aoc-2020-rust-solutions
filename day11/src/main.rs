use std::convert::TryFrom;
use std::io::{self, BufRead};
use std::mem;

fn update_cell_part1(src: &[Vec<u8>], i: usize, j: usize) -> u8 {
    let c = (i.saturating_sub(1)..=i + 1)
        .flat_map(|x| (j.saturating_sub(1)..=j + 1).map(move |y| (x, y)))
        .filter(|&t| t != (i, j))
        .filter_map(|(x, y)| src.get(x)?.get(y))
        .filter(|&c| *c == b'#')
        .count();
    match (src[i][j], c) {
        (b'L', 0) => b'#',
        (b'#', (4..=8)) => b'L',
        _ => src[i][j],
    }
}

fn update_cell_part2(src: &[Vec<u8>], i: usize, j: usize) -> u8 {
    let c = (-1..=1)
        .flat_map(|x| (-1..=1).map(move |y| (x, y)))
        .filter(|&t| t != (0, 0))
        .filter_map(|(x, y)| project(src, i, j, x, y))
        .filter(|&c| c == b'#')
        .count();
    match (src[i][j], c) {
        (b'L', 0) => b'#',
        (b'#', (5..=8)) => b'L',
        _ => src[i][j],
    }
}

fn project(src: &[Vec<u8>], mut i: usize, mut j: usize, x: i32, y: i32) -> Option<u8> {
    let mut current = b'.';
    while current == b'.' {
        i = usize::try_from(i as i32 + x).ok()?;
        j = usize::try_from(j as i32 + y).ok()?;
        current = *src.get(i)?.get(j)?;
    }
    Some(current)
}

fn update_grid(src: &[Vec<u8>], dst: &mut [Vec<u8>], part: usize) {
    for (i, row) in dst.iter_mut().enumerate() {
        for (j, c) in row.iter_mut().enumerate() {
            if *c != b'.' {
                match part {
                    1 => *c = update_cell_part1(src, i, j),
                    2 => *c = update_cell_part2(src, i, j),
                    _ => panic!(),
                }
            }
        }
    }
}

fn run_part(xs: &[Vec<u8>], part: usize) -> usize {
    let mut src = &mut xs.to_vec();
    let mut dst = &mut xs.to_vec();
    update_grid(src, dst, part);
    while src != dst {
        mem::swap(&mut src, &mut dst);
        update_grid(src, dst, part);
    }
    dst.iter().flatten().filter(|&x| *x == b'#').count()
}

fn main() {
    let vec: Vec<_> = io::stdin()
        .lock()
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| x.into_bytes())
        .collect();
    let result = run_part(&vec, 1);
    println!("Part 1: {}", result);
    let result = run_part(&vec, 2);
    println!("Part 2: {}", result);
}

// fn debug_grid(src: &[Vec<u8>]) {
//     for row in src.iter() {
//         println!("{}", String::from_utf8_lossy(row))
//     }
//     println!()
// }
