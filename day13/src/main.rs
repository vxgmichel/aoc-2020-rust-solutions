use std::convert::TryInto;
use std::io::{self, BufRead};

fn modinv(a: isize, module: isize) -> isize {
    let mut mn = (module, a);
    let mut xy = (0, 1);

    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }

    while xy.0 < 0 {
        xy.0 += module;
    }
    xy.0
}

fn part1(n: usize, xs: &[usize]) -> usize {
    let x = xs.iter().min_by_key(|&x| x - n % x).unwrap();
    (x - n % x) * x
}

fn part2(xs: &[(usize, usize)]) -> usize {
    let mut i = 0;
    let mut x = 1;
    let mut k: usize;
    let mut m: usize;
    for &(j, y) in xs {
        m = modinv(x as isize, y as isize).try_into().unwrap();
        k = (i + j) * m % y;
        k = (y - k) % y;
        i += k * x;
        x *= y;
    }
    i
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines().filter_map(|x| x.ok());
    let n: usize = it.next().unwrap().parse().unwrap();
    let xs: Vec<_> = it
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, x)| Some((i, x.parse().ok()?)))
        .collect();
    let xs1: Vec<_> = xs.iter().map(|&(_, x)| x).collect();
    let xs2 = xs;
    let result = part1(n, &xs1);
    println!("Part 1: {}", result);
    let result = part2(&xs2);
    println!("Part 2: {}", result);
}
