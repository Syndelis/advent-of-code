use std::{str::FromStr, collections::HashMap, convert::Infallible};

use strum::EnumString;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    let result = cube_conundrum(INPUT);
    println!("Result: {result}");
}

fn cube_conundrum(input: &str) -> i32 {
    let games = parse_games(input);
    sum_valid_games(games)
}

fn parse_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect()
}

fn sum_valid_games<T: IntoIterator<Item = Game>>(games: T) -> i32 {
    games
        .into_iter()
        .filter(game_is_valid)
        .map(|game| game.id)
        .sum()
}

fn game_is_valid(game: &Game) -> bool {
    for set in &game.sets {
        if set.red > MAX_RED_CUBE_COUNT ||
            set.green > MAX_GREEN_CUBE_COUNT ||
            set.blue > MAX_BLUE_CUBE_COUNT
        {
            return false;
        }
    }

    true
}

const MAX_RED_CUBE_COUNT: i32 = 12;
const MAX_GREEN_CUBE_COUNT: i32 = 13;
const MAX_BLUE_CUBE_COUNT: i32 = 14;

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: i32,
    sets: Vec<Set>,
}

#[derive(Debug, PartialEq, Eq)]
struct Set {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(EnumString, PartialEq, Eq, Hash)]
#[strum(ascii_case_insensitive)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Eq)]
enum NotAGame {
    ImpossibleToSplitAtColon,
    NoId,
}

impl FromStr for Game {
    type Err = NotAGame;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id_part, set_part) = s.split_once(':').ok_or(NotAGame::ImpossibleToSplitAtColon)?;
        let (_, id) = id_part.split_once(' ').ok_or(NotAGame::NoId)?;

        let id = id.parse().map_err(|_| NotAGame::NoId)?;

        let sets: Vec<Set> = set_part
            .split(';')
            .map(Set::from_str)
            .collect::<Result<_, _>>()
            .unwrap();
            

        Ok(Self {
            id,
            sets
        })
    }
}

impl FromStr for Set {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color_count: HashMap<Color, i32> = s
            .split(',')
            .filter_map(|count_and_color| {
                count_and_color.trim().split_once(' ').map(|(count, color)| {
                    (Color::from_str(color).unwrap(), count.parse().unwrap())
                })
            })
            .collect();
        
        Ok(Self {
            red: color_count.get(&Color::Red).copied().unwrap_or_default(),
            green: color_count.get(&Color::Green).copied().unwrap_or_default(),
            blue: color_count.get(&Color::Blue).copied().unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Game, Set, cube_conundrum};
    use test_case::test_case;

    #[test]
    fn example_case() {
        const INPUT: &str = r"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green        
        ";

        const EXPECTED_OUTPUT: i32 = 8;

        let result = cube_conundrum(INPUT);

        assert_eq!(result, EXPECTED_OUTPUT);
    }

    #[test_case(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        Game {
            id: 1,
            sets: vec![
                Set {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                Set {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Set {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ]
        }
    )]
    fn deserialize_game(input_game: &str, expected_output: Game) {
        let result = input_game.parse::<Game>();
        assert_eq!(result, Ok(expected_output));
    }

}