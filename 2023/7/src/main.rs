use std::{str::FromStr, collections::HashMap, num::ParseIntError};

use strum::EnumString;

fn main() -> Result<(), LineParseError> {
    const INPUT: &str = include_str!("../input.txt");
    let result = get_total_winnings(INPUT)?;
    println!("Result: {result}");
    Ok(())
}

fn get_total_winnings(input: &str) -> Result<u64, LineParseError> {
    let mut hands_and_bids: Vec<(Hand, u64)> = input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_hand_and_bids)
        .collect::<Result<_, _>>()?;

    hands_and_bids.sort();

    Ok(hands_and_bids
        .into_iter()
        .enumerate()
        .map(|(idx, (_, bid))| (idx as u64 + 1) * bid)
        .sum()
    )
}

fn parse_hand_and_bids(line: &str) -> Result<(Hand, u64), LineParseError> {
    let (hand_part, bid_part) = line.trim().split_once(' ').unwrap();
    Ok((
        hand_part.trim().parse().map_err(LineParseError::Hand)?,
        bid_part.trim().parse().map_err(LineParseError::Bid)?,
    ))
}

#[derive(Debug)]
enum LineParseError {
    Hand(UnparseableHand),
    Bid(ParseIntError),
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, Hash, Clone, Copy)]
#[strum(ascii_case_insensitive)]
enum Card {
    #[strum(serialize = "2")]
    Two,
    #[strum(serialize = "3")]
    Three,
    #[strum(serialize = "4")]
    Four,
    #[strum(serialize = "5")]
    Five,
    #[strum(serialize = "6")]
    Six,
    #[strum(serialize = "7")]
    Seven,
    #[strum(serialize = "8")]
    Eight,
    #[strum(serialize = "9")]
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
}

impl Hand {
    fn new(cards: [Card; 5]) -> Self {
        let hand_type = HandType::from(&cards);
        Self {
            cards,
            hand_type
        }
    }
}

impl FromStr for Hand {
    type Err = UnparseableHand;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tmp = [0_u8; 4];

        let cards: [Card; 5] = s
            .chars()
            .map(|c| {
                Card::from_str(c.encode_utf8(&mut tmp))
            })
            .collect::<Result<Vec<Card>, _>>()
            .map_err(UnparseableHand::CharNotACard)?
            .try_into()
            .map_err(UnparseableHand::NotFiveElements)?;

        Ok(Self::new(cards))
    }
}

#[derive(Debug)]
enum UnparseableHand {
    CharNotACard(strum::ParseError),
    NotFiveElements(Vec<Card>),
}

impl From<&[Card; 5]> for HandType {
    fn from(value: &[Card; 5]) -> Self {

        let mut card_counts = HashMap::new();

        for card in value {
            *card_counts.entry(card).or_insert(0) += 1;
        }

        // Safety: Guaranteed to have at least 1 element because of the
        // function's signature
        let (card_with_highest_count, highest_count) = unsafe {
            card_counts
                .iter()
                .max_by_key(|(_, count)| *count)
                .map(|(card, count)| (*card, *count))
                .unwrap_unchecked()
        };

        let second_highest_count = card_counts
            .into_iter()
            .filter(|(card, _)| *card != card_with_highest_count)
            .map(|(_, count)| count)
            .max()
            .unwrap_or_default();

        match (highest_count, second_highest_count) {
            (5, _) => Self::FiveOfAKind,
            (4, _) => Self::FourOfAKind,
            (3, 2) => Self::FullHouse,
            (3, _) => Self::ThreeOfAKind,
            (2, 2) => Self::TwoPair,
            (2, _) => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::get_total_winnings;


    const EXAMPLE_INPUT: &str = r"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    ";

    #[test]
    fn example_case() {
        const EXPECTED_OUTPUT: u64 = 6440;
        let result = get_total_winnings(EXAMPLE_INPUT);
        assert_eq!(result.unwrap(), EXPECTED_OUTPUT);
    }
}