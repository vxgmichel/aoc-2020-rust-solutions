use std::convert::TryFrom;
use std::io::{self, BufRead};

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
enum Dir {
    North = 0,
    East,
    South,
    West,
}

impl Dir {
    fn from_usize(n: usize) -> Dir {
        match n % 4 {
            0 => Dir::North,
            1 => Dir::East,
            2 => Dir::South,
            3 => Dir::West,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
enum Move {
    Shift(Dir, usize),
    Turn(usize),
    Forward(usize),
}

impl TryFrom<&str> for Move {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let arg = || value[1..].parse::<usize>().map_err(|e| e.to_string());
        Ok(match value.as_bytes() {
            [b'N', ..] => Move::Shift(Dir::North, arg()?),
            [b'E', ..] => Move::Shift(Dir::East, arg()?),
            [b'S', ..] => Move::Shift(Dir::South, arg()?),
            [b'W', ..] => Move::Shift(Dir::West, arg()?),
            [b'R', ..] => Move::Turn(arg()? / 90),
            [b'L', ..] => Move::Turn((360 - arg()?) / 90),
            [b'F', ..] => Move::Forward(arg()?),
            _ => return Err("Invalid Fromat".to_string()),
        })
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
struct State(Dir, (i32, i32));

impl State {
    fn apply(self, arg: Move) -> Self {
        match arg {
            Move::Shift(d, x) => self.shift(d, x),
            Move::Turn(a) => self.turn(a),
            Move::Forward(x) => self.shift(self.0, x),
        }
    }

    fn turn(self, a: usize) -> Self {
        State(Dir::from_usize(self.0 as usize + a), self.1)
    }

    fn shift(self, d: Dir, x: usize) -> Self {
        let mut p = self.1;
        match d {
            Dir::North => p.0 += x as i32,
            Dir::East => p.1 += x as i32,
            Dir::South => p.0 -= x as i32,
            Dir::West => p.1 -= x as i32,
        }
        State(self.0, p)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
struct State2((i32, i32), (i32, i32));

impl State2 {
    fn apply(self, arg: Move) -> Self {
        match arg {
            Move::Shift(d, x) => self.shift(d, x),
            Move::Turn(a) => self.turn(a),
            Move::Forward(x) => self.forward(x),
        }
    }

    fn forward(self, steps: usize) -> Self {
        let steps = steps as i32;
        let (a, b) = self.1;
        let (i, j) = self.0;
        State2((i, j), (a + steps * i, b + steps * j))
    }

    fn turn(self, a: usize) -> Self {
        let (i, j) = self.0;
        State2(
            match a % 4 {
                0 => (i, j),
                1 => (-j, i),
                2 => (-i, -j),
                3 => (j, -i),
                _ => unreachable!(),
            },
            self.1,
        )
    }

    fn shift(self, d: Dir, x: usize) -> Self {
        let mut p = self.0;
        match d {
            Dir::North => p.0 += x as i32,
            Dir::East => p.1 += x as i32,
            Dir::South => p.0 -= x as i32,
            Dir::West => p.1 -= x as i32,
        }
        State2(p, self.1)
    }
}

fn part1(xs: &[Move]) -> i32 {
    let mut state = State(Dir::East, (0, 0));
    for &x in xs {
        state = state.apply(x);
    }
    let (a, b) = state.1;
    a.abs() + b.abs()
}

fn part2(xs: &[Move]) -> i32 {
    let mut state = State2((1, 10), (0, 0));
    for &x in xs {
        state = state.apply(x);
    }
    let (a, b) = state.1;
    a.abs() + b.abs()
}

fn main() {
    let vec: Vec<Move> = io::stdin()
        .lock()
        .lines()
        .filter_map(|x| Move::try_from(&x.ok()?[..]).ok())
        .collect();
    let result = part1(&vec);
    println!("Part 1: {}", result);
    let result = part2(&vec);
    println!("Part 2: {}", result);
}
