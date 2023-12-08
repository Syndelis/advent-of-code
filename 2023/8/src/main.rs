use std::{collections::HashMap, str::FromStr};
use nom::{IResult, sequence::{separated_pair, delimited}, character::complete::{alphanumeric1, char, multispace0}, bytes::complete::is_not, combinator::map_parser, error::ParseError};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use strum::EnumString;

fn main() -> Result<(), NavigationError<'static>> {
    const INPUT: &str = include_str!("../input.txt");
    let result = navigate_map_part_2(INPUT)?;
    println!("Result: {result}");

    Ok(())
}

fn navigate_map_part_2(input: &str) -> Result<u64, NavigationError> {
    let (navigator, map) = parse_intructions_and_map(input)
        .map_err(NavigationError::InputParserError)?;

    let steps: Vec<u64> = map
        .par_iter()
        .filter(|(key, _)| key.ends_with('A'))
        .map(|(key, _)| {
            let mut navigator = navigator.clone();
            let mut current_position = key;

            while !current_position.ends_with('Z') {
                let map_pos = map.get(current_position)
                    .ok_or_else(|| NavigationError::PositionDoesNotExist(current_position.to_owned()))?;

                current_position = navigator
                    .direction()
                    .index(map_pos);
            }

            Ok(navigator.steps_taken)
        })
        .collect::<Result<_, _>>()?;

    steps
        .into_iter()
        .reduce(lcm)
        .ok_or_else(|| unreachable!("There ought to be an element for every thread"))

}

fn lcm(a: u64, b: u64) -> u64 {
    let (hi, lo) = if a > b {
        (a, b)
    } else {
        (b, a)
    };

    let x = (hi)..=(hi*lo);
    x
        .step_by(hi as usize)
        .find(|i| i % lo == 0)
        .unwrap_or(1)
}

fn navigate_map(input: &str) -> Result<u64, NavigationError> {
    let (mut navigator, map) = parse_intructions_and_map(input)
        .map_err(NavigationError::InputParserError)?;

    let mut current_position = "AAA";
    const TARGET_POSITION: &str = "ZZZ";

    while current_position != TARGET_POSITION {
        let map_pos = map.get(current_position)
            .ok_or_else(|| NavigationError::PositionDoesNotExist(current_position.to_owned()))?;

        current_position = navigator
            .direction()
            .index(map_pos)
            .as_ref();
    }

    Ok(navigator.steps_taken)
}

#[derive(Debug)]
enum NavigationError<'s> {
    PositionDoesNotExist(String),
    InputParserError(InputParseError<'s>),
}

type Map = HashMap<String, (String, String)>;

fn parse_intructions_and_map(input: &str) -> Result<(Navigator, Map), InputParseError> {
    let (navigator, map) = input
        .trim()
        .split_once("\n\n")
        .ok_or(InputParseError::NoNavigationMapDivision)?;

    let navigator = Navigator::from_str(navigator)
        .map_err(InputParseError::NavigatorParseError)?;

    let map = parse_map(map)
        .map_err(InputParseError::MapParseError)?;

    Ok((navigator, map))
}

fn parse_map(map: &str) -> Result<Map, LineParseError> {
    map
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_map_node)
        .collect()
}

fn parse_map_node(line: &str) -> Result<(String, (String, String)), LineParseError> {
    fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        F: Fn(&'a str) -> IResult<&'a str, O, E>,
    {
        delimited(
            multispace0,
            inner,
            multispace0
        )
    }

    fn parse_left_right(input: &str) -> IResult<&str, (&str, &str)> {
        let parenthesized = delimited(char('('), is_not(")"), char(')'));
        let two_word_parser = separated_pair(ws(alphanumeric1), char(','), ws(alphanumeric1));

        map_parser(
            parenthesized,
            two_word_parser
        )(input)
    }

    fn parse_origin_destination(input: &str) -> IResult<&str, (&str, (&str, &str))> {
        separated_pair(
            ws(alphanumeric1),
            char('='),
            ws(parse_left_right)
        )(input)
    }

    let (_, (origin, (left, right))) = parse_origin_destination(line)
        .map_err(LineParseError)?;

    Ok((origin.to_owned(), (left.to_owned(), right.to_owned())))

}

#[derive(Debug)]
struct LineParseError<'s>(nom::Err<nom::error::Error<&'s str>>);

#[derive(Debug)]
enum InputParseError<'s> {
    NoNavigationMapDivision,
    NavigatorParseError(NavigatorParseError),
    MapParseError(LineParseError<'s>),
}

#[derive(Clone, Copy, EnumString)]
enum Instruction {
    L,
    R,
}

impl Instruction {
    fn index<'t, T>(&self, (left, right): &'t (T, T)) -> &'t T {
        match self {
            Instruction::L => left,
            Instruction::R => right,
        }
    }
}

#[derive(Clone)]
struct Navigator {
    instructions: Vec<Instruction>,
    current: usize,
    steps_taken: u64,
}

impl Navigator {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            current: 0,
            steps_taken: 0,
        }
    }

    fn direction(&mut self) -> &Instruction {
        let instruction = &self.instructions[self.current];
        self.current = (self.current + 1) % self.instructions.len();
        self.steps_taken += 1;

        instruction
    }
}

impl FromStr for Navigator {
    type Err = NavigatorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tmp = [0_u8; 4];

        let instructions: Vec<Instruction> = s
            .chars()
            .map(|c| {
                Instruction::from_str(c.encode_utf8(&mut tmp))
            })
            .collect::<Result<_, _>>()
            .map_err(NavigatorParseError::InvalidInstruction)?;

        Ok(Self::new(instructions))
    }
}

#[derive(Debug)]
enum NavigatorParseError {
    InvalidInstruction(strum::ParseError),
}

#[cfg(test)]
mod tests {
    use crate::{parse_map_node, navigate_map, navigate_map_part_2};
    use test_case::test_case;

    #[test_case(
        r"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        ",
        2
    )]
    #[test_case(
        r"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)        
        ",
        6
    )]
    fn example_case(input: &str, expected_steps_taken: u64) {
        let result = navigate_map(input).unwrap();
        assert_eq!(result, expected_steps_taken);
    }

    #[test]
    fn example_case_part_2() {
        const INPUT: &str = r"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        ";

        const EXPECTED_OUTPUT: u64 = 6;

        let result = navigate_map_part_2(INPUT).unwrap();

        assert_eq!(result, EXPECTED_OUTPUT);
    }

    #[test_case("AAA = (BBB, CCC)", "AAA", "BBB", "CCC")]
    #[test_case("  GGG   =   ( HHH  , III)  ", "GGG", "HHH", "III")]
    fn line_parser(line: &str, expected_origin: &str, expected_left: &str, expected_right: &str) {
        let (origin, (left, right)) = parse_map_node(line).unwrap();
        assert_eq!(origin, expected_origin);
        assert_eq!(left, expected_left);
        assert_eq!(right, expected_right);
    }
}