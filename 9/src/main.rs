use std::{ops::{Add, AddAssign}, str::FromStr, num::ParseIntError, collections::HashSet, cmp::max};

fn main() {
    let mut input: Vec<Movement> =
        include_str!("../input.txt")
        .lines()
        .map(|s| Movement::from_str(s).unwrap())
        .collect();

    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };

    let mut tail_visited_positions: HashSet<Position> = HashSet::new();
    tail_visited_positions.insert(tail);

    for movement in input.iter_mut() {
        while let Some(delta) = movement.take_step() {
            process_movement(&mut head, &mut tail, &delta);
            tail_visited_positions.insert(tail);
        }
    }

    println!("Tail visited positions: {:?}", tail_visited_positions.len());

}

fn process_movement(head: &mut Position, tail: &mut Position, delta: &Position) {
    let new_head = *head + *delta;

    if tail.distance_from(&new_head) > 1 {
        *tail = *head;
    }

    *head = new_head;
}


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {x: i32, y: i32}

impl Position {
    fn distance_from(&self, other: &Self) -> u32 {
        // diagonals count as 1
        let x = (self.x - other.x).unsigned_abs();
        let y = (self.y - other.y).unsigned_abs();

        max(x, y)
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Movement {
    direction: Direction,
    amount: u32,
}

impl Movement {
    fn take_step(&mut self) -> Option<Position> {
        if self.amount == 0 {
            return None;
        }

        self.amount -= 1;

        let pos = match self.direction {
            Direction::Up => Position { x: 0, y: 1 },
            Direction::Down => Position { x: 0, y: -1 },
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
        };

        Some(pos)
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone)]
enum MovementParsingError {
    StringCantBeSplit,
    IntegerParsing(ParseIntError),
    DirectionParsing(DirecitonParsingError),
}

impl FromStr for Movement {
    type Err = MovementParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amnt) = s.split_once(' ').ok_or(Self::Err::StringCantBeSplit)?;

        let dir: Direction = dir.parse().map_err(Self::Err::DirectionParsing)?;
        let amnt: u32 = amnt.parse().map_err(Self::Err::IntegerParsing)?;

        Ok(Self { direction: dir, amount: amnt })
    }
}

#[derive(Debug, Clone, Copy)]
enum DirecitonParsingError {
    InvalidDirection,
    EmptyString,
}

impl FromStr for Direction {
    type Err = DirecitonParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().ok_or(Self::Err::EmptyString)? {
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(Self::Err::InvalidDirection),
        }
    }
}