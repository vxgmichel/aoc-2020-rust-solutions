use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
enum Operation {
    Mem {
        address: u64,
        value: u64,
    },
    Mask {
        and_mask: u64,
        or_mask: u64,
        mask: String,
    },
}

fn parse(s: String) -> Option<Operation> {
    match s.get(0..4)? {
        "mask" => parse_mask(s),
        "mem[" => parse_mem(s),
        _ => None,
    }
}

fn parse_mask(s: String) -> Option<Operation> {
    let mut and_mask = (0..28).map(|_| "1").collect::<String>();
    let mut or_mask = (0..28).map(|_| "0").collect::<String>();
    and_mask.push_str(&s.get(7..)?.replace("X", "1"));
    or_mask.push_str(&s.get(7..)?.replace("X", "0"));
    Some(Operation::Mask {
        and_mask: u64::from_str_radix(&and_mask, 2).ok()?,
        or_mask: u64::from_str_radix(&or_mask, 2).ok()?,
        mask: s.get(7..)?.to_string(),
    })
}

fn parse_mem(s: String) -> Option<Operation> {
    let mut it = s.get(4..)?.split("] = ");
    Some(Operation::Mem {
        address: it.next()?.parse().ok()?,
        value: it.next()?.parse().ok()?,
    })
}

fn part1(xs: &[Operation]) -> u64 {
    let mut memory = HashMap::new();
    let mut current_and_mask: u64 = 0;
    let mut current_or_mask: u64 = 0;
    for x in xs {
        match x {
            Operation::Mask {
                and_mask,
                or_mask,
                mask: _,
            } => {
                current_and_mask = *and_mask;
                current_or_mask = *or_mask;
            }
            Operation::Mem { address, mut value } => {
                value &= current_and_mask;
                value |= current_or_mask;
                *memory.entry(address).or_insert(0) = value;
            }
        }
    }
    memory.values().sum()
}

fn part2(xs: &[Operation]) -> u64 {
    let mut memory = HashMap::new();
    let mut current_mask = &String::new();
    for x in xs {
        match x {
            Operation::Mask {
                and_mask: _,
                or_mask: _,
                mask,
            } => current_mask = &mask,
            Operation::Mem { address, value } => {
                write_part2(&mut memory, *address, *value, &current_mask, 0)
            }
        }
    }
    memory.values().sum()
}

fn write_part2(
    memory: &mut HashMap<u64, u64>,
    address: u64,
    value: u64,
    mask: &str,
    final_address: u64,
) {
    match mask.chars().last() {
        None => *memory.entry(final_address).or_insert(0) = value,
        Some('0') => write_part2(
            memory,
            address / 2,
            value,
            &mask[..mask.len() - 1],
            2 * final_address + address % 2,
        ),
        Some('1') => write_part2(
            memory,
            address / 2,
            value,
            &mask[..mask.len() - 1],
            2 * final_address + 1,
        ),
        Some('X') => {
            write_part2(
                memory,
                address / 2,
                value,
                &mask[..mask.len() - 1],
                2 * final_address,
            );
            write_part2(
                memory,
                address / 2,
                value,
                &mask[..mask.len() - 1],
                2 * final_address + 1,
            );
        }
        _ => panic!(),
    }
}

fn main() {
    let stdin = io::stdin();
    let xs: Vec<_> = stdin
        .lock()
        .lines()
        .filter_map(|x| parse(x.ok()?))
        .collect();
    let result = part1(&xs);
    println!("Part 1: {}", result);
    let result = part2(&xs);
    println!("Part 2: {}", result);
}
