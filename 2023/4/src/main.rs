use std::{collections::{HashSet, HashMap}, num::ParseIntError};

fn main() -> Result<(), CardParseError> {
    const INPUT: &str = include_str!("../input.txt");
    let result = scratchcards_part_2(INPUT)?;
    println!("Result: {result}");

    Ok(())
}

fn scratchcards(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .filter_map(calculate_points)
        .sum()
}

fn scratchcards_part_2(input: &str) -> Result<u32, CardParseError> {
    let mut largest_existing_id = 0;
    let mut copis_of_cards = HashMap::<u32, u32>::new();

    for card_line in input.trim().lines().filter(|line| !line.is_empty()) {
        let (id_part, numbers_part) = card_line
            .trim()
            .split_once(':')
            .ok_or(CardParseError::MissingDelimiter(':'))?;

        let id: u32 = id_part
            .strip_prefix("Card ")
            .ok_or(CardParseError::MissingPrefix)?
            .trim()
            .parse()
            .map_err(|e| CardParseError::MissingId(e, id_part.to_owned()))?;

        largest_existing_id = largest_existing_id.max(id);

        let (winning_numbers, my_numbers) = numbers_part
            .trim()
            .split_once('|')
            .ok_or(CardParseError::MissingDelimiter('|'))?;

        let intersect_count = hits(winning_numbers, my_numbers);

        let my_copies = *copis_of_cards.entry(id).or_insert(1);

        for card_id_offset in 1..=intersect_count {
            let card_incr_id = id + card_id_offset;
            *copis_of_cards.entry(card_incr_id).or_insert(1) += my_copies;
        }
    }

    Ok(
        copis_of_cards
            .into_iter()
            .filter(|(id, _)| *id <= largest_existing_id)
            .map(|(_, count)| count)
            .sum()
    )

}

#[derive(Debug, PartialEq, Eq)]
enum CardParseError {
    MissingDelimiter(char),
    MissingPrefix,
    MissingId(ParseIntError, String),
}

fn calculate_points(line: &str) -> Option<u32> {
    line
        .trim()
        .split_once(':')
        .unwrap()
        .1
        .split_once('|')
        .and_then(|(winning_numbers, my_numbers)| {
        
        let intersect_count = hits(winning_numbers, my_numbers);

        (intersect_count >= 1)
            .then_some(2_u32.pow((intersect_count as i32 - 1).max(0) as u32))
    })
}

fn hits(winning_numbers: &str, my_numbers: &str) -> u32 {
    let winning_numbers = parse_numbers(winning_numbers);
    let my_numbers = parse_numbers(my_numbers);

    winning_numbers
        .intersection(&my_numbers).count() as u32
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
    use crate::{scratchcards, calculate_points, scratchcards_part_2};
    use test_case::test_case;

    const EXAMPLE_INPUT: &str = r"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ";

    #[test]
    fn example_case() {
        const EXPECTED_OUTPUT: u32 = 13;

        let result = scratchcards(EXAMPLE_INPUT);

        assert_eq!(result, EXPECTED_OUTPUT);
    }

    #[test]
    fn example_case_part_2() {
        const EXPECTED_OUTPUT: u32 = 30;

        let result = scratchcards_part_2(EXAMPLE_INPUT);

        assert_eq!(result, Ok(EXPECTED_OUTPUT));
    }

    #[test_case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", Some(8))]
    fn test_calculate_points(card: &str, expected_points: Option<u32>) {
        let result = calculate_points(card);
        assert_eq!(result, expected_points);
    }
}
