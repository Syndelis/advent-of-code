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

    fn find_shape_based_on_result(self, result: &RpsResult) -> RpsShape {
        // Finds the shape X that satisfies `X vs self = result`

        if matches!(result, RpsResult::Draw) {
            return self;
        }

        match (self, result) {
            | (RpsShape::Rock, RpsResult::Win)
            | (RpsShape::Scissors, RpsResult::Lose) => RpsShape::Paper,

            | (RpsShape::Rock, RpsResult::Lose)
            | (RpsShape::Paper, RpsResult::Win) => RpsShape::Scissors,

            
            | (RpsShape::Paper, RpsResult::Lose)
            | (RpsShape::Scissors, RpsResult::Win) => RpsShape::Rock,

            _ => unreachable!()
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
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => RpsShape::Rock,
            "B" | "Y" => RpsShape::Paper,
            "C" | "Z" => RpsShape::Scissors,
            _ => panic!("Invalid shape {s}"),
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

impl From<&str> for RpsResult {
    fn from(s: &str) -> Self {
        match s {
            "X" => RpsResult::Lose,
            "Y" => RpsResult::Draw,
            "Z" => RpsResult::Win,
            _ => panic!("Invalid result {s}"),
        }
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

        (theirs, mine)
    });

    // Part 1
    let total_score: i32 = games.clone().map(|(theirs, mine): (&str, &str)| -> i32 {
        let theirs: RpsShape = theirs.into();
        let mine: RpsShape = mine.into();
        i32::from(mine.result_against(&theirs)) + mine.shape_bonus()
    }).sum();

    println!("Total score (part 1): {total_score}");

    // Part 2
    let total_score: i32 = games.map(|(theirs, game_result): (&str, &str)| -> i32 {
        let theirs: RpsShape = theirs.into();
        let game_result: RpsResult = game_result.into();
        
        let mine = theirs.find_shape_based_on_result(&game_result);

        i32::from(game_result) + mine.shape_bonus()
    }).sum();

    println!("Total score (part 2): {total_score}");

}
