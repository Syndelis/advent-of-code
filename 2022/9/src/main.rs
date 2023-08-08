#![feature(get_many_mut)]

use std::{ops::{Add, AddAssign, Sub}, str::FromStr, num::ParseIntError, collections::HashSet, cmp::max};

fn main() {
    let mut input: Vec<Movement> =
        include_str!("../test_input2.txt")
        .lines()
        .map(|s| Movement::from_str(s).unwrap())
        .collect();

    // Part 1

    {

        let mut input = input.clone();

        let mut head = Position { x: 0, y: 0 };
        let mut tail = Position { x: 0, y: 0 };
    
        let mut tail_visited_positions: HashSet<Position> = HashSet::new();
        tail_visited_positions.insert(tail);
    
        for movement in input.iter_mut() {
            while let Some(delta) = movement.take_step() {
                (head, tail) = process_movement(head, tail, &delta);
                tail_visited_positions.insert(tail);
            }
        }
    
        println!("Tail visited positions: {:?}", tail_visited_positions.len());
    }

    // Part 2

    let mut rope = [Position { x: 0, y: 0 }; 10];

    let mut tail_visited_positions: HashSet<Position> = HashSet::new();
    tail_visited_positions.insert(*rope.last().unwrap());

    for movement in input.iter_mut() {
        while let Some(delta) = movement.take_step() {
            {
                let (head, body) = rope.split_at_mut(1);
                (head[0], body[0]) = process_movement(head[0], body[0], &delta);
            }

            let idx_pairs = (1..rope.len() - 1).zip(2..rope.len());

            for (idx, next_idx) in idx_pairs {
                let [prev, next] = rope.get_many_mut([idx, next_idx]).unwrap();
                if prev.distance_from(next) > 1 {
                    *next += next.close_gap_from(prev, &movement.direction);
                }
            }

            tail_visited_positions.insert(*rope.last().unwrap());
        }
    }

    println!("Long tail visited positions: {:?}", tail_visited_positions.len());

    // let expected_test_input1 = [
    //     (2, 2),
    //     (1, 2),
    //     (2, 2),
    //     (3, 2),
    //     (2, 2),
    //     (1, 1),
    //     (0, 0),
    //     (0, 0),
    //     (0, 0),
    //     (0, 0),
    // ].iter().map(|(x, y)| Position { x: *x, y: *y }).collect::<Vec<_>>();

    let expected_test_input2 = [
        ()
    ].iter().map(|(x, y)| Position { x: *x, y: *y }).collect::<Vec<_>>();

    assert_eq!(Vec::from(rope), expected_test_input2);

    println!("Rope: {rope:?}");

}

fn process_movement(head: Position, tail: Position, delta: &Position) -> (Position, Position) {
    let new_head = head + *delta;

    if tail.distance_from(&new_head) > 1 {
        (new_head, head)
    }
    
    else {
        (new_head, tail)
    }

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

    fn close_gap_from(&self, other: &Self, dir: &Direction) -> Self {
        // Returns the delta to move `self` in order for it to be at most
        // 1 unit away from `other`. The delta is at most 1 unit in each axis.

        let delta_in_axis: Position = (*dir).into();

        delta_in_axis + match dir.axis().opposite() {
            Axis::Horizontal => {
                let x = (other.x - self.x).signum();
                Position { x, y: 0 }
            },
            Axis::Vertical => {
                let y = (other.y - self.y).signum();
                Position { x: 0, y }
            },
        }

    }

    fn difference_on_axis(&self, other: &Self, axis: &Axis) -> Option<Movement> {
        let (delta, direction_options) = match axis {
            Axis::Horizontal => (self.x - other.x, [Direction::Left, Direction::Right]),
            Axis::Vertical => (self.y - other.y, [Direction::Up, Direction::Down]),
        };

        let direction = match delta {
            x if x < 0 => direction_options[0],
            x if x > 0 => direction_options[1],
            _ => return None,
        };

        Some(Movement {
            direction,
            amount: delta.unsigned_abs(),
        })
    }
}

impl From<Direction> for Position {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Self { x: 0, y: 1 },
            Direction::Down => Self { x: 0, y: -1 },
            Direction::Left => Self { x: -1, y: 0 },
            Direction::Right => Self { x: 1, y: 0 },
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn axis(&self) -> Axis {
        match self {
            Self::Up | Self::Down => Axis::Vertical,
            Self::Left | Self::Right => Axis::Horizontal,
        }
    }
}

#[derive(Clone, Copy)]
enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    fn opposite(self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
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

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
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