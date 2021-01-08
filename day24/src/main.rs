use std::collections::HashMap;
use std::io::{self, BufRead};

type Tile = (i32, i32);

fn add_tile((x1, y1): Tile, (x2, y2): Tile) -> Tile {
    (x1 + x2, y1 + y2)
}

const E_TILE: Tile = (2, 0);
const NE_TILE: Tile = (1, 1);
const NW_TILE: Tile = (-1, 1);
const W_TILE: Tile = (-2, 0);
const SW_TILE: Tile = (-1, -1);
const SE_TILE: Tile = (1, -1);

fn string_to_tile(s: &str) -> Tile {
    if s == "" {
        return (0, 0);
    }
    match &s[0..1] {
        "e" | "w" => add_tile(
            string_to_tile(&s[1..]),
            match &s[0..1] {
                "e" => E_TILE,
                "w" => W_TILE,
                _ => panic!(),
            },
        ),
        "n" | "s" => add_tile(
            string_to_tile(&s[2..]),
            match &s[0..2] {
                "ne" => NE_TILE,
                "nw" => NW_TILE,
                "se" => SE_TILE,
                "sw" => SW_TILE,
                _ => panic!(),
            },
        ),
        _ => panic!(),
    }
}

fn part1(tiles: &[Tile]) -> usize {
    let mut counter: HashMap<Tile, usize> = HashMap::new();
    for &t in tiles {
        *counter.entry(t).or_default() ^= 1
    }
    counter.values().sum()
}

fn part2(tiles: &[Tile]) -> usize {
    let mut counter: HashMap<Tile, usize> = HashMap::new();
    for &t in tiles {
        *counter.entry(t).or_default() ^= 1
    }
    counter.retain(|_, &mut v| v == 1);
    for _ in 0..100 {
        let minx = *counter.keys().map(|(x, _)| x).min().unwrap() - 2;
        let maxx = *counter.keys().map(|(x, _)| x).max().unwrap() + 2;
        let miny = *counter.keys().map(|(_, y)| y).min().unwrap() - 1;
        let maxy = *counter.keys().map(|(_, y)| y).max().unwrap() + 1;
        let reference = counter.clone();
        for x in minx..=maxx {
            for y in miny..=maxy {
                if (x + y) % 2 == 0 {
                    let tile = (x, y);
                    let value = reference.get(&tile).cloned().unwrap_or_default();
                    let neighbors: usize = [E_TILE, NE_TILE, NW_TILE, W_TILE, SW_TILE, SE_TILE]
                        .iter()
                        .map(|&x| {
                            reference
                                .get(&add_tile(tile, x))
                                .cloned()
                                .unwrap_or_default()
                        })
                        .sum();
                    match (value, neighbors) {
                        (0, 2) => counter.insert(tile, 1),
                        (1, 0) => counter.remove(&tile),
                        (1, 3..=6) => counter.remove(&tile),
                        _ => None,
                    };
                }
            }
        }
    }
    counter.len()
}

fn main() {
    let tiles: Vec<Tile> = io::stdin()
        .lock()
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| string_to_tile(&x))
        .collect();
    let result = part1(&tiles);
    println!("Part 1: {}", result);
    let result = part2(&tiles);
    println!("Part 2: {}", result);
}
