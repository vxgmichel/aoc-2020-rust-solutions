use std::collections::HashSet;
use std::convert::TryFrom;
use std::io::{self, BufRead};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Op {
    ACC,
    JMP,
    NOP,
}

impl TryFrom<&str> for Op {
    type Error = &'static str;

    fn try_from(c: &str) -> Result<Self, Self::Error> {
        match c.as_bytes() {
            b"acc" => Ok(Op::ACC),
            b"jmp" => Ok(Op::JMP),
            b"nop" => Ok(Op::NOP),
            _ => Err("Not a valid op"),
        }
    }
}

fn parse(s: String) -> Option<(Op, i32)> {
    match s.split(' ').collect::<Vec<_>>()[..] {
        [first, second] => Some((Op::try_from(first).ok()?, second.parse().ok()?)),
        _ => None,
    }
}

fn part1(xs: &[(Op, i32)]) -> i32 {
    rec_part1(xs, (0, 0), &mut HashSet::new())
}

fn rec_part1(xs: &[(Op, i32)], state: (i32, i32), set: &mut HashSet<i32>) -> i32 {
    let (index, acc) = state;
    if set.contains(&index) {
        return acc;
    }
    set.insert(index);
    let (op, value) = xs[index as usize];
    let state = match op {
        Op::ACC => (index + 1, acc + value),
        Op::JMP => (index + value, acc),
        Op::NOP => (index + 1, acc),
    };
    rec_part1(xs, state, set)
}

fn part2(xs: &[(Op, i32)]) -> i32 {
    rec_part2(xs, (0, 0), &mut HashSet::new(), false).unwrap()
}

fn rec_part2(
    xs: &[(Op, i32)],
    state: (i32, i32),
    set: &mut HashSet<i32>,
    switched: bool,
) -> Option<i32> {
    let (index, acc) = state;
    if set.contains(&index) {
        return None;
    }
    if index == xs.len() as i32 {
        return Some(acc);
    }
    set.insert(index);
    let (op, value) = xs[index as usize];
    let subresult = match op {
        Op::ACC => rec_part2(xs, (index + 1, acc + value), set, switched),
        Op::JMP => rec_part2(xs, (index + value, acc), set, switched),
        Op::NOP => rec_part2(xs, (index + 1, acc), set, switched),
    };
    if subresult.is_some() || switched {
        return subresult;
    }
    //let set = &mut set.clone();
    match op {
        Op::ACC => None,
        Op::JMP => rec_part2(xs, (index + 1, acc), set, true),
        Op::NOP => rec_part2(xs, (index + value, acc), set, true),
    }
}

fn main() {
    let vec: Vec<_> = io::stdin()
        .lock()
        .lines()
        .filter_map(|x| parse(x.ok()?))
        .collect();
    let result = part1(&vec);
    println!("Part 1: {}", result);
    let result = part2(&vec);
    println!("Part 2: {}", result);
}
