#![feature(let_chains)]

use std::{str::FromStr, num::ParseIntError};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use strum::FromRepr;

fn main() -> Result<(), InputParseError> {
    const INPUT: &str = include_str!("../input.txt");
    let result = sum_possible_combinations(INPUT)?;
    println!("Result: {result}");
    Ok(())
}

fn sum_possible_combinations(input: &str) -> Result<u64, InputParseError> {
    Ok(parse_spring_conditions(input)?
        .into_iter()
        .map(|(groups, expected_counts)| process_group(&groups, 0, &expected_counts))
        .sum()
    )
}

fn process_group(
    springs: &[Spring], // ???.### -> (???, ###)
    current_count: u64, // +1
    expected_counts: &[u64], // (1, 1, 3)
) -> u64
{
    static INVALID_STATE: fn() -> u64 = || 0;

    if let [last_expected] = expected_counts && *last_expected == current_count {
        if springs
            .iter()
            .filter(|s| matches!(s, Spring::Damaged))
            .count() == 0
        {
            return 1;
        } else {
            return 0;
        }
    }

    match (springs.len(), expected_counts.len()) {
        (0, 0) => return INVALID_STATE(),
        (0, _) | (_, 0) => return INVALID_STATE(),
        _ => {}
    }

    let curr_spring = &springs[0];
    let curr_expected = expected_counts[0];

    let dot = || {
        if current_count == 0 {
            process_group(&springs[1..], 0, expected_counts)
        } else if current_count == curr_expected {
            process_group(&springs[1..], 0, &expected_counts[1..])
        } else {
            INVALID_STATE()
        }
    };

    let hash = || {
        if current_count + 1 > curr_expected {
            INVALID_STATE()
        } else {
            process_group(&springs[1..], current_count + 1, expected_counts)
        }
    };

    match curr_spring {
        Spring::Functioning => dot(),
        Spring::Damaged => hash(),
        Spring::Unknown => {
            dot() + hash()
        },
    }
}

fn parse_spring_conditions(input: &str) -> Result<Vec<(Vec<Spring>, Vec<u64>)>, InputParseError> {
    input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (group_part, expected_part) = line
                .trim()
                .split_once(' ')
                .ok_or(InputParseError::NoWhitespaceSplit)?;

            let springs = group_part
                .trim()
                .chars()
                .map(|c| Spring::from_repr(c as u64))
                .collect::<Option<_>>()
                .ok_or(InputParseError::InvalidSpring)?;

            let expected_damaged = expected_part
                .trim()
                .split(',')
                .map(u64::from_str)
                .collect::<Result<_, _>>()
                .map_err(InputParseError::InvalidCount)?;

            Ok((springs, expected_damaged))

        })
        .collect()
}

#[repr(u64)]
#[derive(Debug, FromRepr)]
enum Spring {
    Unknown = '?' as u64,
    Damaged = '#' as u64,
    Functioning = '.' as u64,
}

#[derive(Debug)]
enum InputParseError {
    InvalidCount(ParseIntError),
    InvalidSpring,
    NoWhitespaceSplit,
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{parse_spring_conditions, process_group, sum_possible_combinations};

    const EXAMPLE_INPUT: &str = r"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1    
    ";

    #[test]
    fn example_case() {
        const EXPECTED_OUTPUT: u64 = 21;
        let result = sum_possible_combinations(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, EXPECTED_OUTPUT);
    }

    #[test_case("???.### 1,1,3", 1)]
    #[test_case(".??..??...?##. 1,1,3", 4)]
    #[test_case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[test_case("????.#...#... 4,1,1", 1)]
    #[test_case("????.######..#####. 1,6,5", 4)]
    #[test_case("?###???????? 3,2,1", 10)]
    #[test_case("........ 0", 1)]
    #[test_case(".....?... 1", 1)]
    #[test_case("########## 10", 1)]
    #[test_case("?######### 10", 1 ; "another 10")]
    #[test_case("?????????? 1", 10)]
    #[test_case("?????????? 2", 9)]
    #[test_case("??.?. 1,1", 2)]
    #[test_case("?.?.?. 1,1", 3)]
    #[test_case("?#..???..##? 2,3,3", 1)]
    #[test_case(".???#??#??.? 7,1", 4)]
    #[test_case(".#??.????. 2,2", 3)]
    #[test_case("?.# 1", 1 ; "edge?")]
    fn test_group_processing(input: &str, expected_variations: u64) {
        let ins = parse_spring_conditions(input).unwrap();

        let mut variations_sum = 0;

        for (groups, expected_counts) in ins {
           let variation_count = process_group(&groups, 0, &expected_counts);
           variations_sum += variation_count;
        }

        assert_eq!(variations_sum, expected_variations);
    }
}