use std::io::{self, BufRead};

const BASE: u64 = 7;
const MODULUS: u64 = 20201227;

fn reverse(a: u64, base: u64, modulus: u64) -> u64 {
    let mut current = 1;
    for x in 0.. {
        if current == a {
            return x;
        };
        current *= base;
        current %= modulus;
    }
    unreachable!()
}

fn modpow(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    for _ in 0..exp {
        result *= base;
        result %= modulus;
    }
    result
}

fn main() {
    let args = io::stdin()
        .lock()
        .lines()
        .map(|x| x.ok()?.parse().ok())
        .collect::<Option<Vec<_>>>()
        .unwrap();
    let a: u64 = args[0];
    let b: u64 = args[1];
    let x = reverse(a, BASE, MODULUS);
    let result = modpow(b, x, MODULUS);
    println!("Part 1: {}", result);
    println!("Done \\o/");
}
