use std::{str::FromStr, collections::HashMap};

use indicatif::{ParallelProgressIterator, ProgressIterator};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() -> Result<(), InvalidLineState> {
    const INPUT: &str = include_str!("../input.txt");
    let result = lowest_location_part_2(INPUT)?;
    println!("Result: {result}");
    Ok(())
}

fn lowest_location(input: &str) -> Result<u64, InvalidLineState> {
    let (mut seed_val, maps) = parse_seeds_and_maps(input)?;

    let mut current_map_name = "seed";

    while current_map_name != "location" {
        let current_map = maps
            .get(current_map_name)
            .ok_or_else(|| InvalidLineState::NonExistingMap(current_map_name.to_owned()))?;

        current_map_name = &current_map.to;

        seed_val
            .iter_mut()
            .for_each(|s| *s = current_map.map_slot(*s));
    }

    Ok(seed_val.into_iter().min().unwrap())
}

fn lowest_location_part_2(input: &str) -> Result<u64, InvalidLineState> {
    let (seed_ranges, maps) = parse_seeds_and_maps(input)?;

    let mut min = u64::MAX;

    for seed_range in seed_ranges.chunks(2).progress_count((seed_ranges.len() / 2) as u64) {
        let &[range_start, range_len] = seed_range else {
            continue;
        };

        min = min.min((range_start..(range_start + range_len))
            .into_par_iter()
            .progress_count(range_len)
            .filter_map(|mut seed| {
                let mut current_map_name = "seed";
                
                while current_map_name != "location" {
                    let current_map = maps.get(current_map_name)?;

                    current_map_name = &current_map.to;
                    seed = current_map.map_slot(seed);
                }

                Some(seed)
            })
            .min()
            .unwrap_or(u64::MAX)
        );
    }


    Ok(min)
}

fn parse_seeds_and_maps(input: &str) -> Result<(Vec<u64>, HashMap<String, Mapping>), InvalidLineState> {
    let mut seeds = Vec::new();
    let mut maps = HashMap::new();
    let mut current_map = None;

    for line in input.lines().map(|line| line.trim()).filter(|line| !line.is_empty()) {
        let action = line
            .parse()
            .map_err(InvalidLineState::LineActionError)?;

        match action {
            LineAction::StoreSeeds(s) => seeds = s,
            LineAction::StartMapping { from, to } => {
                maps.insert(from.clone(), Mapping {
                    to: to.clone(),
                    maps: Vec::new(),
                });

                current_map = Some(from);
            },
            LineAction::StoreMapRule(new_map) => {
                let Some(ref current_map) = current_map else {
                    return Err(InvalidLineState::InsertBeforeMapDeclaration);
                };

                let mapping = maps
                    .get_mut(current_map)
                    .ok_or_else(|| InvalidLineState::NonExistingMap(current_map.to_owned()))?;

                mapping.maps.push(new_map);
            },
        }
    }

    Ok((seeds, maps))
}

#[derive(Debug, PartialEq, Eq)]
enum InvalidLineState {
    InsertBeforeMapDeclaration,
    NonExistingMap(String),
    LineActionError(InvalidLine),
}

#[derive(Debug)]
enum LineAction {
    StoreSeeds(Vec<u64>),
    StartMapping { from: String, to: String },
    StoreMapRule(MapRule),
}

impl FromStr for LineAction {
    type Err = InvalidLine;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(seed_str) = s.strip_prefix("seeds: ") {
            let seeds = seed_str
                .split_whitespace()
                .map(|seed_val| seed_val.parse())
                .collect::<Result<_, _>>()
                .map_err(|_| InvalidLine::UnparseableSeeds(s.to_owned()))?;

            Ok(LineAction::StoreSeeds(seeds))
        }
        else if let Ok(map) = MapRule::from_str(s) {
            Ok(LineAction::StoreMapRule(map))
        } else if let Some((from, to)) = s
            .strip_suffix(" map:")
            .and_then(|map_name| map_name.split_once("-to-"))
        {
            Ok(LineAction::StartMapping { from: from.to_owned(), to: to.to_owned() })
        } else {
            unreachable!("A line must always conform to one of the previous formats!")
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum InvalidLine {
    UnparseableSeeds(String),
}

struct Mapping {
    to: String,
    maps: Vec<MapRule>,
}

trait MapSlot<T> {
    fn map_slot(&self, val: T) -> T;
}

impl MapSlot<u64> for Mapping {
    fn map_slot(&self, val: u64) -> u64 {
        for MapRule { dest_start, src_start, range_len, } in &self.maps {
            if val >= *src_start && val <= *src_start + *range_len {
                return (val - *src_start) + *dest_start
            }
        }

        val
    }
}

#[derive(Debug)]
struct MapRule {
    dest_start: u64,
    src_start: u64,
    range_len: u64,
}

#[derive(Debug)]
enum NotAMapRule {
    Unparseable(String),
    ValueCountDifference(String, Vec<u64>),
}

impl FromStr for MapRule {
    type Err = NotAMapRule;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals: Vec<u64> = s
            .split_whitespace()
            .map(|val| val.trim().parse())
            .collect::<Result<_, _>>()
            .map_err(|_| NotAMapRule::Unparseable(s.to_owned()))?;

        let [dest_start, src_start, range_len]: [u64; 3] = vals
            .try_into()
            .map_err(|vals| NotAMapRule::ValueCountDifference(s.to_owned(), vals))?;

        Ok(Self {
            dest_start,
            src_start,
            range_len,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{lowest_location, lowest_location_part_2};


    const EXAMPLE_INPUT: &str = r"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    ";

    #[test]
    fn example_case() {
        const EXPECTED_OUTPUT: u64 = 35;
        let result = lowest_location(EXAMPLE_INPUT);
        assert_eq!(result, Ok(EXPECTED_OUTPUT));
    }

    #[test]
    fn example_case_part_2() {
        const EXPECTED_OUTPUT: u64 = 46;
        let result = lowest_location_part_2(EXAMPLE_INPUT);
        assert_eq!(result, Ok(EXPECTED_OUTPUT));
    }
}
