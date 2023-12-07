use std::{str::FromStr, collections::HashMap, num::ParseIntError, cmp::Ordering};

use strum::EnumString;

fn main() -> Result<(), LineParseError> {
    const INPUT: &str = include_str!("../input.txt");
    let result = get_total_winnings_part_2(INPUT)?;
    println!("Result: {result}");
    Ok(())
}

fn get_total_winnings_part_2(input: &str) -> Result<u64, LineParseError> {
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
    J,
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
    Q,
    K,
    A,
}

impl Card {
    fn is_joker(&self) -> bool {
        matches!(self, Card::J)
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

        let jokers_count = card_counts.get(&Card::J).copied().unwrap_or_default();

        let (card_with_highest_count, highest_count) = card_counts
                .iter()
                .filter(|(card, _)| !card.is_joker())
                .max_by_key(|(_, count)| *count)
                .map(|(card, count)| (*card, *count))
                .unwrap_or((&Card::J, 0));

        let second_highest_count = card_counts
            .into_iter()
            .filter(|(card, _)| {
                !card.is_joker() && *card != card_with_highest_count
            })
            .map(|(_, count)| count)
            .max()
            .unwrap_or_default();

        match (highest_count + jokers_count, second_highest_count) {
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
    use std::str::FromStr;

    use crate::{get_total_winnings_part_2, HandType, Hand};
    use test_case::test_case;

    const EXAMPLE_INPUT: &str = r"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    ";

    #[test]
    fn example_case_part_2() {
        const EXPECTED_OUTPUT: u64 = 5905;
        let result = get_total_winnings_part_2(EXAMPLE_INPUT);
        assert_eq!(result.unwrap(), EXPECTED_OUTPUT);
    }

    #[test_case("4558J", HandType::ThreeOfAKind)]
    #[test_case("T7JJT", HandType::FourOfAKind)]
    #[test_case("AAJJJ", HandType::FiveOfAKind)]
    #[test_case("9J2TT", HandType::ThreeOfAKind)]
    #[test_case("T8JTJ", HandType::FourOfAKind)]
    #[test_case("6J69J", HandType::FourOfAKind)]
    #[test_case("4J935", HandType::OnePair)]
    #[test_case("JJJ8J", HandType::FiveOfAKind)]
    #[test_case("222J2", HandType::FiveOfAKind)]
    #[test_case("JKKKJ", HandType::FiveOfAKind)]
    #[test_case("QJ533", HandType::ThreeOfAKind)]
    #[test_case("666JJ", HandType::FiveOfAKind)]
    #[test_case("AA9J7", HandType::ThreeOfAKind)]
    #[test_case("QJ777", HandType::FourOfAKind)]
    #[test_case("JJJJJ", HandType::FiveOfAKind)]
    fn test_hand_type_with_jokers(line: &str, expected_type: HandType) {
        let hand = Hand::from_str(line).unwrap();
        assert_eq!(hand.hand_type, expected_type);
    }
}