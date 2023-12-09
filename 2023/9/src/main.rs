#![feature(iter_map_windows)]

use std::{str::FromStr, num::ParseIntError};

fn main() -> Result<(), SequenceParseError> {
    const INPUT: &str = include_str!("../input.txt");
    let result = sum_predictions(INPUT)?;
    println!("Result: {result}");
    Ok(())
}

fn sum_predictions(input: &str) -> Result<i64, SequenceParseError> {
    input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(predict_sequence)
        .sum()
}

fn predict_sequence(line: &str) -> Result<i64, SequenceParseError> {
    let mut values: Vec<i64> = line
        .split_whitespace()
        .map(FromStr::from_str)
        .collect::<Result<_, _>>()
        .map_err(SequenceParseError::InvalidValue)?;

    let mut all_zeroes = false;
    let mut last_elements = Vec::with_capacity(values.len() - 1);

    while !all_zeroes {
        let mut cur_last_element = 0;

        all_zeroes = true;

        values = values
            .into_iter()
            .inspect(|el| cur_last_element = *el)
            .map_windows(|[a, b]| b - a)
            .inspect(|sub| all_zeroes &= *sub == 0)
            .collect();

        last_elements.push(cur_last_element);
    }

    Ok(last_elements.into_iter().sum())

}

#[derive(Debug)]
enum SequenceParseError {
    InvalidValue(ParseIntError),
}

#[cfg(test)]
mod tests {
    use crate::sum_predictions;


    const EXAMPLE_INPUT: &str = r"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    ";

    #[test]
    fn example_case() {
        const EXPECTED_OUTPUT: i64 = 114;
        let result = sum_predictions(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, EXPECTED_OUTPUT);
    }
}