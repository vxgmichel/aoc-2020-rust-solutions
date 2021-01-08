use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};

type Deck = VecDeque<usize>;

fn parse_player(lines: &[String]) -> Deck {
    lines[1..lines.len()]
        .iter()
        .filter_map(|x| x.parse().ok())
        .collect()
}

fn play_game(xs1: &Deck, xs2: &Deck) -> (Deck, Deck) {
    let mut xs1 = xs1.clone();
    let mut xs2 = xs2.clone();
    while !xs1.is_empty() && !xs2.is_empty() {
        if let (Some(x1), Some(x2)) = (xs1.pop_front(), xs2.pop_front()) {
            match x1.cmp(&x2) {
                Ordering::Less => {
                    xs2.push_back(x2);
                    xs2.push_back(x1)
                }
                Ordering::Greater => {
                    xs1.push_back(x1);
                    xs1.push_back(x2)
                }
                Ordering::Equal => panic!(),
            }
        }
    }
    (xs1, xs2)
}

fn part1(xs1: &Deck, xs2: &Deck) -> usize {
    let (r1, r2) = play_game(xs1, xs2);
    let result = if r1.is_empty() { r2 } else { r1 };
    result
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (i + 1) * x)
        .sum()
}

fn play_recursive_game(xs1: &Deck, xs2: &Deck) -> (Deck, Deck) {
    let mut xs1 = xs1.clone();
    let mut xs2 = xs2.clone();
    let mut seen: HashSet<(Deck, Deck)> = HashSet::new();
    while !xs1.is_empty() && !xs2.is_empty() {
        if !seen.insert((xs1.clone(), xs2.clone())) {
            xs2.clear();
            break;
        }

        if let (Some(x1), Some(x2)) = (xs1.pop_front(), xs2.pop_front()) {
            match x1.cmp(&x2) {
                _ if x1 <= xs1.len() && x2 <= xs2.len() => {
                    let subxs1: Deck = xs1.iter().take(x1).copied().collect();
                    let subxs2: Deck = xs2.iter().take(x2).copied().collect();
                    assert!(subxs1.len() == x1);
                    assert!(subxs2.len() == x2);
                    let (r1, r2) = play_recursive_game(&subxs1, &subxs2);
                    if r1.is_empty() {
                        xs2.push_back(x2);
                        xs2.push_back(x1)
                    } else if r2.is_empty() {
                        xs1.push_back(x1);
                        xs1.push_back(x2)
                    } else {
                        panic!()
                    }
                }
                Ordering::Less => {
                    xs2.push_back(x2);
                    xs2.push_back(x1)
                }
                Ordering::Greater => {
                    xs1.push_back(x1);
                    xs1.push_back(x2)
                }
                Ordering::Equal => panic!(),
            }
        }
    }
    (xs1, xs2)
}

fn part2(xs1: &Deck, xs2: &Deck) -> usize {
    let (r1, r2) = play_recursive_game(xs1, xs2);
    let result = if r1.is_empty() { r2 } else { r1 };
    result
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (i + 1) * x)
        .sum()
}

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().filter_map(|x| x.ok()).collect();
    let mut it = lines.split(|x| x.is_empty());
    let player1 = parse_player(it.next().unwrap());
    let player2 = parse_player(it.next().unwrap());
    let result = part1(&player1, &player2);
    println!("Part 1: {}", result);
    let result = part2(&player1, &player2);
    println!("Part 2: {}", result);
}
