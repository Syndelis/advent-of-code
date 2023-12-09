#![feature(iter_map_windows)]
#![feature(iterator_try_reduce)]

use std::{str::FromStr, num::ParseIntError};

fn main() -> Result<(), SequenceParseError> {
    const INPUT: &str = include_str!("../input.txt");
    let (past, future) = sum_predictions(INPUT)?;
    println!("Past: {past} | Future: {future}");
    Ok(())
}

fn sum_predictions(input: &str) -> Result<(i64, i64), SequenceParseError> {
    input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(predict_sequence)
        .try_fold((0, 0), |(acc_past, acc_future), val| {
            let (past, future) = val?;
            Ok((acc_past + past, acc_future + future))
        })
}

fn predict_sequence(line: &str) -> Result<(i64, i64), SequenceParseError> {
    let mut values: Vec<i64> = line
        .split_whitespace()
        .map(FromStr::from_str)
        .collect::<Result<_, _>>()
        .map_err(SequenceParseError::InvalidValue)?;

    let mut all_zeroes = false;
    let mut last_elements = Vec::with_capacity(values.len() - 1);
    let mut first_elements = Vec::with_capacity(values.len() - 1);

    while !all_zeroes {
        all_zeroes = true;
        
        first_elements.push(values.first().copied().unwrap_or_default());
        last_elements.push(values.last().copied().unwrap_or_default());

        values = values
            .into_iter()
            .map_windows(|[a, b]| b - a)
            .inspect(|sub| all_zeroes &= *sub == 0)
            .collect();
    }

    let past = first_elements
        .into_iter()
        .rev()
        .reduce(|a, b| b - a)
        .ok_or_else(|| unreachable!("It is guaranteed to have been populated with elements"))?;

    let future = last_elements
        .into_iter()
        .sum();

    Ok((past, future))

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
        const EXPECTED_PAST: i64 = 2;
        const EXPECTED_FUTURE: i64 = 114;
        let result = sum_predictions(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, (EXPECTED_PAST, EXPECTED_FUTURE));
    }
}