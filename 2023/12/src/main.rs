#![feature(let_chains)]

use std::{str::FromStr, num::ParseIntError, ops::Range, collections::HashMap};

use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelIterator, ParallelIterator, IntoParallelRefMutIterator};
use strum::FromRepr;

fn main() -> Result<(), InputParseError> {
    const INPUT: &str = include_str!("../input.txt");
    let result = sum_possible_combinations(INPUT, Part::Two)?;
    println!("Result: {result}");
    Ok(())
}

fn sum_possible_combinations(input: &str, part: Part) -> Result<u64, InputParseError> {
    let mut conditions = parse_spring_conditions(input)?;

    if let Part::Two = part {
        explode_conditions(&mut conditions);
    }

    
    #[cfg(not(test))]
    {
        let total = conditions.len();
        Ok(conditions
            .into_par_iter()
            .progress_count(total as u64)
            .map(|(springs, expected_counts)| {
                let damage_windows = get_damage_windows(&springs);
                let mut cache = HashMap::new();
                process_group(&springs, 0, 0, 0, &expected_counts, &damage_windows, &mut cache)
            })
            .sum()
        )
    }
    #[cfg(test)]
    {
        Ok(conditions
            .into_iter()
            .map(|(springs, expected_counts)| {
                let damage_windows = get_damage_windows(&springs);
                let mut cache = HashMap::new();
                process_group(&springs, 0, 0, 0, &expected_counts, &damage_windows, &mut cache)
            })
            .sum()
        )
    }
}

fn get_damage_windows(springs: &[Spring]) -> Vec<Range<usize>> {
    let mut max_damage = Vec::new();

    let mut last_idx = 0;
    let stop_at_idx = springs.len();

    while last_idx < stop_at_idx {
        let idx = springs[last_idx..]
            .iter()
            .enumerate()
            .find(|(_, s)| matches!(s, Spring::Unknown | Spring::Damaged))
            .map(|(idx, _)| idx + last_idx)
            .unwrap_or(springs.len() - 1);

        let last = springs[idx..]
            .iter()
            .enumerate()
            .find(|(_, s)| matches!(s, Spring::Functioning))
            .map(|(this_idx, _)| this_idx + idx)
            .unwrap_or(springs.len() - 1);

        max_damage.push(idx..last);

        last_idx = last + 1;
    }

    max_damage
}

fn explode_conditions(conditions: &mut Vec<Condition>) {
    conditions
        .par_iter_mut()
        .for_each(|(springs, expected_counts)| {
            let orig_springs = springs.clone();
            let orig_expected_counts = expected_counts.clone();

            (0..4).for_each(|_| {
                springs.push(Spring::Unknown);
                springs.extend(orig_springs.iter());

                expected_counts.extend(orig_expected_counts.iter());
            });
        });
}

fn process_group(
    springs: &[Spring], // ???.### -> (???, ###)
    mut springs_idx: usize,
    mut current_count: u64, // +1
    expected_idx: usize,
    mut expected_counts: &[u64], // (1, 1, 3)
    damage_windows: &[Range<usize>],
    cache: &mut HashMap<(usize, usize, u64), u64>
) -> u64
{
    let cache_key = (springs_idx, expected_idx, current_count);

    if let Some(res) = cache.get(&cache_key) {
        return *res;
    }

    let mut cached = |cache: &mut HashMap<_, _>, val| {
        cache.insert(cache_key, val);
        val
    };

    let mut invalid_state = |cache| cached(cache, 0);

    let future_damaged_springs = springs[springs_idx..].len();

    let shifted_expected_counts = &expected_counts[expected_idx..];

    let expected_springs = shifted_expected_counts.iter().sum::<u64>() as i64 + shifted_expected_counts.len() as i64 - 1;
    if (current_count as i64) + (future_damaged_springs as i64) < expected_springs {
        return invalid_state(cache);
    }

    if let [last_expected] = shifted_expected_counts && *last_expected == current_count {
        if springs[springs_idx..]
            .iter()
            .filter(|s| matches!(s, Spring::Damaged))
            .count() == 0
        {
            return 1;
        } else {
            return 0;
        }
    }

    match (springs.len() - springs_idx, shifted_expected_counts.len()) {
        (0, 0) => return invalid_state(cache),
        (0, _) | (_, 0) => return invalid_state(cache),
        _ => {}
    }

    let curr_expected = shifted_expected_counts[0];

    let curr_spring = &springs[springs_idx];

    let dot_result = if matches!(curr_spring, Spring::Functioning | Spring::Unknown) {
        if current_count == 0 {
            process_group(springs, springs_idx + 1, 0, expected_idx, expected_counts, damage_windows, cache)
        } else if current_count == curr_expected {
            process_group(springs, springs_idx + 1, 0, expected_idx + 1, expected_counts, damage_windows, cache)
        } else {
            0
        }
    } else {
        0
    };

    let hash_result = if matches!(curr_spring, Spring::Damaged | Spring::Unknown) {
        if current_count + 1 > curr_expected {
            0
        } else if current_count > 0
            && let Some(damage_window) = damage_windows
                .iter()
                .find(|window| window.contains(&springs_idx))
        {
            let jump_length = (curr_expected as isize - current_count as isize).min(damage_window.end as isize - springs_idx as isize);
            if jump_length > 0 {
                process_group(springs, springs_idx + jump_length as usize, current_count + jump_length as u64, expected_idx, expected_counts, damage_windows, cache)
            } else {
                process_group(springs, springs_idx + 1, current_count + 1, expected_idx, expected_counts, damage_windows, cache)
            }
        } else {
            process_group(springs, springs_idx + 1, current_count + 1, expected_idx, expected_counts, damage_windows, cache)
        }
    } else {
        0
    };

    cached(cache, dot_result + hash_result)
}

type Condition = (Vec<Spring>, Vec<u64>);

fn parse_spring_conditions(input: &str) -> Result<Vec<Condition>, InputParseError> {
    input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (group_part, expected_part) = line
                .trim()
                .split_once(' ')
                .ok_or(InputParseError::NoWhitespaceSplit)?;

            let springs: Vec<Spring> = group_part
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
#[derive(Debug, FromRepr, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Unknown = '?' as u64,
    Damaged = '#' as u64,
    Functioning = '.' as u64,
}

enum Part {
    One,
    Two,
}

#[derive(Debug)]
enum InputParseError {
    InvalidCount(ParseIntError),
    InvalidSpring,
    NoWhitespaceSplit,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use test_case::test_case;

    use crate::{parse_spring_conditions, process_group, sum_possible_combinations, Part, explode_conditions, Spring, get_damage_windows};

    const EXAMPLE_INPUT: &str = r"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1    
    ";

    #[test_case(Part::One, 21)]
    #[test_case(Part::Two, 525152)]
    fn example_case(part: Part, expected_count: u64) {
        let result = sum_possible_combinations(EXAMPLE_INPUT, part).unwrap();
        assert_eq!(result, expected_count);
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
        let conditions = parse_spring_conditions(input).unwrap();

        let mut variations_sum = 0;

        for (springs, expected_counts) in conditions {
            let damage_windows = get_damage_windows(&springs);
            let mut cache = HashMap::new();
            let variation_count = process_group(&springs, 0, 0, 0, &expected_counts, &damage_windows, &mut cache);
            variations_sum += variation_count;
        }

        assert_eq!(variations_sum, expected_variations);
    }

    use Spring::*;

    #[test_case(
        ".# 1",
        // ".#?.#?.#?.#?.#",
        &[
            Functioning, Damaged, Unknown,
            Functioning, Damaged, Unknown,
            Functioning, Damaged, Unknown,
            Functioning, Damaged, Unknown,
            Functioning, Damaged
        ],
        // "1,1,1,1,1"
        &[1, 1, 1, 1, 1]
    )]
    fn test_explode_conditions(input: &str, expected_springs: &[Spring], expected_counts: &[u64]) {
        let mut conditions = parse_spring_conditions(input).unwrap();
        explode_conditions(&mut conditions);

        let [(springs, counts)] = &conditions[..] else { unreachable!() };

        assert_eq!(springs, expected_springs);
        assert_eq!(counts, expected_counts);
    }
}