use std::{fs::File, io::Read};

#[derive(PartialEq)]
enum RpsShape {
    Rock,
    Paper,
    Scissors,
}

enum RpsResult {
    Win,
    Lose,
    Draw,
}

impl RpsShape {
    fn result_against(&self, other: &RpsShape) -> RpsResult {

        if self == other {
            RpsResult::Draw
        }

        else if matches!(
            (self, other),
            | (RpsShape::Rock, RpsShape::Scissors)
            | (RpsShape::Paper, RpsShape::Rock)
            | (RpsShape::Scissors, RpsShape::Paper)
        ) {
            RpsResult::Win
        }

        else {
            RpsResult::Lose
        }
    }

    fn shape_bonus(&self) -> i32 {
        match self {
            RpsShape::Rock => 1,
            RpsShape::Paper => 2,
            RpsShape::Scissors => 3,
        }
    }
}

impl From<&str> for RpsShape {
    fn from(c: &str) -> Self {
        match c {
            "A" | "X" => RpsShape::Rock,
            "B" | "Y" => RpsShape::Paper,
            "C" | "Z" => RpsShape::Scissors,
            _ => panic!("Invalid shape"),
        }
    }
}

impl From<RpsResult> for i32 {
    fn from(r: RpsResult) -> Self {
        match r {
            RpsResult::Win => 6,
            RpsResult::Lose => 0,
            RpsResult::Draw => 3,
        }
    }
}

impl RpsResult {
    fn battle<I, J>(first: I, second: J) -> Self
    where
        I: Into<RpsShape>,
        J: Into<RpsShape>,
    {
        first.into().result_against(&second.into())
    }
}

fn main() {
    
    let mut input = String::new();
    
    File::open("input.txt")
        .expect("Failed to open input file")
        .read_to_string(&mut input)
        .expect("Couldn't read the file into a string");

    let games = input.split_terminator('\n').map(|game_str| {
        let game_str: Vec<&str> = game_str.split_whitespace().collect();

        assert!(game_str.len() == 2, "Invalid game string: {}", game_str.join(" "));

        let theirs = game_str[0];
        let mine = game_str[1];

        (theirs.into(), mine.into())
    });

    // Part 1
    let total_score: i32 = games.map(|(theirs, mine): (RpsShape, RpsShape)| -> i32 {
        i32::from(mine.result_against(&theirs)) + mine.shape_bonus()
    }).sum();

    println!("Total score: {total_score}");

}
