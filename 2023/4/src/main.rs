use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    let result = scratchcards(INPUT);
    println!("Result: {result}");
}

fn scratchcards(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .filter_map(calculate_points)
        .sum()
}

fn calculate_points(line: &str) -> Option<u32> {
    line
        .trim()
        .split_once(':')
        .unwrap()
        .1
        .split_once('|')
        .and_then(|(winning_numbers, my_numbers)| {
        let winning_numbers = parse_numbers(winning_numbers);
        let my_numbers = parse_numbers(my_numbers);

        let intersetction_numbers = winning_numbers
            .intersection(&my_numbers).count() as u32;

        (intersetction_numbers >= 1)
            .then_some(2_u32.pow((intersetction_numbers as i32 - 1).max(0) as u32))
    })
}

fn parse_numbers(numbers: &str) -> HashSet<u32> {
    numbers
        .split_whitespace()
        .map(|number| number.parse())
        .collect::<Result<_, _>>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{scratchcards, calculate_points};
    use test_case::test_case;

    #[test]
    fn example_case() {
        const INPUT: &str = r"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";

        const EXPECTED_OUTPUT: u32 = 13;

        let result = scratchcards(INPUT);

        assert_eq!(result, EXPECTED_OUTPUT);
    }

    #[test_case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", Some(8))]
    fn test_calculate_points(card: &str, expected_points: Option<u32>) {
        let result = calculate_points(card);
        assert_eq!(result, expected_points);
    }
}
