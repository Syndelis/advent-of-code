use std::str::FromStr;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    let result = calculate_numbers_of_ways_to_beat_the_only_race(INPUT);
    println!("Result: {result}");
}

fn calculate_numbers_of_ways_to_beat_the_only_race(input: &str) -> u64 {
    let (time_str, distance_str) = input.trim().split_once('\n').unwrap();
    
    let time: u64 = time_str
        .trim()
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .fold(String::new(), |mut acc, val| {
            acc.push_str(val);
            acc
        })
        .parse()
        .unwrap();

    let distance: u64 = distance_str
        .trim()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .fold(String::new(), |mut acc, val| {
            acc.push_str(val);
            acc
        })
        .parse()
        .unwrap();

    amount_of_ways_to_win_boat_race((time, distance))
}

fn calculate_numbers_of_ways_to_beat_races(input: &str) -> u64 {
    let times_and_distances = parse_times_and_distances(input);

    times_and_distances
        .into_par_iter()
        .map(amount_of_ways_to_win_boat_race)
        .product()
}

fn amount_of_ways_to_win_boat_race((max_time, max_dist): (u64, u64)) -> u64 {
    let [min, max] = solve_boat_equation(max_time, max_dist);
    max - min + 1
}

fn parse_times_and_distances(input: &str) -> Vec<(u64, u64)> {
    let (time_str, distance_str) = input.trim().split_once('\n').unwrap();
    
    let times = time_str
        .trim()
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|val| val.parse().unwrap());

    let distances = distance_str
        .trim()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|val| val.parse().unwrap());

    times
        .zip(distances)
        .collect()
}

fn solve_boat_equation(max_time: u64, max_distance: u64) -> [u64; 2] {
    let a = - (ACCELERATION_RATE as f64);
    let b = max_time as f64;
    let c = - (max_distance as f64);

    let [lo, hi] = solve_second_degree_equation(a, b, c);

    [
        lo.floor() as u64 + 1,
        hi.ceil() as u64 - 1,
    ]
}

fn solve_second_degree_equation(a: f64, b: f64, c: f64) -> [f64; 2] {
    let a2 = a * 2_f64;
    let common = (b.powf(2_f64) - 4_f64 * a * c).sqrt();

    [
        (-b + common) / a2,
        (-b - common) / a2
    ]
}

const ACCELERATION_RATE: u64 = 1; // 1 millimeter / (1 millisecond ^ 2)

#[cfg(test)]
mod tests {
    use crate::{calculate_numbers_of_ways_to_beat_races, solve_boat_equation, calculate_numbers_of_ways_to_beat_the_only_race};
    use test_case::test_case;

    const EXAMPLE_INPUT: &str = r"
        Time:      7  15   30
        Distance:  9  40  200    
    ";

    #[test]
    fn example_case() {
        const EXPECTED_OUTPUT: u64 = 288;
        let result = calculate_numbers_of_ways_to_beat_races(EXAMPLE_INPUT);
        assert_eq!(result, EXPECTED_OUTPUT);
    }

    #[test]
    fn example_case_part_2() {
        const EXPECTED_OUTPUT: u64 = 71503;
        let result = calculate_numbers_of_ways_to_beat_the_only_race(EXAMPLE_INPUT);
        assert_eq!(result, EXPECTED_OUTPUT);
    }

    #[test_case(7, 9, [2, 5])]
    #[test_case(15, 40, [4, 11])]
    #[test_case(30, 200, [11, 19])]
    fn boat_eq(max_time: u64, max_distance: u64, expected_solutions: [u64; 2]) {
        let result = solve_boat_equation(max_time, max_distance);
        assert_eq!(result, expected_solutions);
    }
}