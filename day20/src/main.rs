use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

type Tile = Vec<Vec<u8>>;
type Image = Vec<Vec<u8>>;
type Transformation = (bool, bool, bool);
type Edge = Vec<u8>;
type Slot = (Option<Edge>, Option<Edge>);

fn parse_tile(xs: &[String]) -> (usize, Tile) {
    let head = &xs[0];
    let tail = &xs[1..xs.len()];
    let tid = head[5..head.len() - 1].parse().unwrap();
    let tile = tail
        .iter()
        .map(|x| x.chars().map(|x| (x == '#') as u8).collect())
        .collect();
    (tid, tile)
}

fn transform_tile(src: &Tile, hvd: Transformation) -> Tile {
    let (h, v, d) = hvd;
    let mut dest = if d {
        (0..src[0].len())
            .map(|x| src.iter().map(|r| r[x]).collect())
            .collect()
    } else {
        src.to_vec()
    };
    if h {
        dest.reverse();
    }
    if v {
        dest.iter_mut().for_each(|x| x.reverse());
    }
    dest
}

fn count_edges(tiles: &HashMap<usize, Tile>) -> HashMap<Edge, usize> {
    let mut result = HashMap::new();
    for tile in tiles.values() {
        for &h in &[false, true] {
            for &v in &[false, true] {
                for &d in &[false, true] {
                    let tile = transform_tile(&tile, (h, v, d));
                    *result.entry(tile[0].clone()).or_default() += 1;
                }
            }
        }
    }
    result
}

fn find_connections(tiles: &HashMap<usize, Tile>) -> HashSet<Edge> {
    count_edges(tiles)
        .iter()
        .filter(|&(_, v)| *v > 1)
        .map(|(k, _)| k.clone())
        .collect()
}

fn build_slot_to_tile(
    tiles: &HashMap<usize, Tile>,
    connections: &HashSet<Edge>,
) -> HashMap<Slot, Vec<(usize, Tile)>> {
    let mut result = HashMap::new();
    for (&tid, tile) in tiles.iter() {
        for &h in &[false, true] {
            for &v in &[false, true] {
                for &d in &[false, true] {
                    let tile = transform_tile(&tile, (h, v, d));
                    let left: Edge = (0..tile[0].len()).map(|x| tile[x][0]).collect();
                    let some_up = connections.get(&tile[0]).cloned();
                    let some_left = connections.get(&left).cloned();
                    let key = (some_up, some_left);
                    result.entry(key).or_insert_with(Vec::new).push((tid, tile));
                }
            }
        }
    }
    result
}

fn build_tile_grid(
    size: usize,
    slot_to_tile: &HashMap<Slot, Vec<(usize, Tile)>>,
) -> Vec<Vec<(usize, Tile)>> {
    let mut result: Vec<Vec<(usize, Tile)>> = Vec::new();
    let mut seen: HashSet<usize> = HashSet::new();
    for i in 0..size {
        let mut current: Vec<(usize, Tile)> = Vec::new();
        for j in 0..size {
            let some_up = if i == 0 {
                None
            } else {
                let (_, up_tile) = &result[i - 1][j];
                let down_edge = up_tile[up_tile.len() - 1].clone();
                Some(down_edge)
            };
            let some_left = if j == 0 {
                None
            } else {
                let (_, left_tile) = &current[j - 1];
                let length = left_tile.len();
                let right_edge: Edge = (0..length).map(|x| left_tile[x][length - 1]).collect();
                Some(right_edge)
            };
            let key = (some_up, some_left);
            let value = slot_to_tile
                .get(&key)
                .unwrap()
                .iter()
                .find(|(x, _)| !seen.contains(x))
                .unwrap();
            seen.insert(value.0);
            current.push(value.clone());
        }
        result.push(current);
    }
    result
}

fn part1(tile_grid: &[Vec<(usize, Tile)>]) -> usize {
    let size = tile_grid.len();
    tile_grid[0][0].0
        * tile_grid[0][size - 1].0
        * tile_grid[size - 1][0].0
        * tile_grid[size - 1][size - 1].0
}

fn build_image(tile_grid: &[Vec<(usize, Tile)>]) -> Image {
    let mut result = Vec::new();
    let tile_size = tile_grid[0][0].1.len();
    for row in tile_grid {
        for tile_i in 1..tile_size - 1 {
            let mut current = Vec::new();
            for (_, tile) in row {
                let subrow = &tile[tile_i];
                current.extend_from_slice(&subrow[1..tile_size - 1])
            }
            result.push(current);
        }
    }
    result
}

#[allow(dead_code)]
fn show_image(image: &Image) {
    for row in image {
        let s = row
            .iter()
            .map(|&x| if x == 0 { ' ' } else { '█' })
            .collect::<String>();
        println!("{}", s);
    }
    println!();
}

fn match_at_position(x: usize, y: usize, image: &Image, pattern: &Image) -> bool {
    for (i, row) in pattern.iter().enumerate() {
        for (j, &chr) in row.iter().enumerate() {
            if chr == 1 && image[x + i][y + j] == 0 {
                return false;
            }
        }
    }
    true
}

fn count_matches(image: &Image, pattern: &Image) -> usize {
    let mut result = 0;
    let stop_x = image.len() - pattern.len() + 1;
    let stop_y = image.len() - pattern[0].len() + 1;
    for x in 0..stop_x {
        for y in 0..stop_y {
            if match_at_position(x, y, image, pattern) {
                result += 1;
            }
        }
    }
    result
}

fn part2(image: &Image, pattern: &Image) -> usize {
    let total_image: usize = image.iter().flatten().map(|&x| x as usize).sum();
    let total_pattern: usize = pattern.iter().flatten().map(|&x| x as usize).sum();
    for &h in &[false, true] {
        for &v in &[false, true] {
            for &d in &[false, true] {
                let image = transform_tile(image, (h, d, v));
                let count = count_matches(&image, pattern);
                if count != 0 {
                    return total_image - total_pattern * count;
                }
            }
        }
    }
    total_image
}

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().filter_map(|x| x.ok()).collect();
    let tiles: HashMap<usize, Tile> = lines.split(|x| x == "").map(|x| parse_tile(x)).collect();
    let size: usize = (tiles.len() as f64).sqrt() as usize;
    let connections = find_connections(&tiles);
    let slot_to_tile = build_slot_to_tile(&tiles, &connections);
    let tile_grid = build_tile_grid(size, &slot_to_tile);
    let result = part1(&tile_grid);
    println!("Part 1: {}", result);

    let image = build_image(&tile_grid);
    let pattern = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1],
        vec![0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0],
    ];
    // show_image(&image);
    // show_image(&pattern);
    let result = part2(&image, &pattern);
    println!("Part 2: {}", result);
}
