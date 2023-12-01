#![feature(custom_test_frameworks)]

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    let result = calibration_values(INPUT);

    println!("Result: {result}");
}

fn calibration_values(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {            
            let (first_digit, last_digit) = fetch_first_and_last_digits(line);
            (first_digit * 10 + last_digit) as u64
        })
        .sum()
}

fn fetch_first_and_last_digits(line: &str) -> (u8, u8) {
    let (first_digit, _) = fetch_digit(line, DigitIdxSorting::First).unwrap_or_default();

    let last_digit = match fetch_digit(line, DigitIdxSorting::Last) {
        Some((last_digit, _)) => last_digit,
        None => first_digit,
    };

    (first_digit, last_digit)
}

#[repr(i8)]
#[derive(Clone, Copy)]
enum DigitIdxSorting {
    First = -1,
    Last = 1,
}

fn fetch_digit(line: &str, sorting: DigitIdxSorting) -> Option<(u8, usize)> {
    static DIGIT_NAMES_AND_VALUES: &[(&str, u8)] = &[
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    if let Some((at, digit)) = DIGIT_NAMES_AND_VALUES
        .iter()
        .filter_map(|(name, value)| {
            match sorting {
                DigitIdxSorting::First => line.find(name),
                DigitIdxSorting::Last => line.rfind(name),
            }.map(|at| (at, value))
        })
        .max_by_key(|(at, _)| (*at as i64) * (sorting as i64))
    {
            return Some((*digit, at));
    }

    None
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    use crate::{calibration_values, fetch_digit, DigitIdxSorting, fetch_first_and_last_digits};

    #[test]
    fn base_case() {
        let example_input = r"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        ";

        let expected_output = 142;

        let output = calibration_values(example_input);

        assert_eq!(output, expected_output);
    }

    #[test_case("1", Some((1, 0)) ; "single digit")]
    #[test_case("one", Some((1, 0)) ; "single named digit")]
    #[test_case("31", Some((3, 0)) ; "multiple digits")]
    #[test_case("fourtwo", Some((4, 0)) ; "multiple named digits")]
    #[test_case("2six", Some((2, 0)) ; "mixed digit first")]
    #[test_case("zero8", Some((0, 0)) ; "mixed named digit first")]
    #[test_case("alotoftextfivemoretext", Some((5, 10)) ; "trash surrounding named digit")]
    #[test_case("nothing here!", None ; "no digits")]
    fn test_fetch_first_digit(input_line: &str, expected_output: Option<(u8, usize)>) {
        let output = fetch_digit(input_line, DigitIdxSorting::First);
        assert_eq!(output, expected_output);
    }

    #[test_case("1", Some((1, 0)) ; "single digit")]
    #[test_case("one", Some((1, 0)) ; "single named digit")]
    #[test_case("31", Some((1, 1)) ; "multiple digits")]
    #[test_case("fourtwo", Some((2, 4)) ; "multiple named digits")]
    #[test_case("2six", Some((6, 1)) ; "mixed digit first")]
    #[test_case("zero8", Some((8, 4)) ; "mixed named digit first")]
    #[test_case("alotoftextfivemoretext", Some((5, 10)) ; "trash surrounding named digit")]
    #[test_case("nothing here!", None ; "no digits")]
    fn test_fetch_last_digit(input_line: &str, expected_output: Option<(u8, usize)>) {
        let output = fetch_digit(input_line, DigitIdxSorting::Last);
        assert_eq!(output, expected_output);
    }

    #[test_case("8eight1", 8, 1)]
    #[test_case("98126", 9, 6)]
    #[test_case("fourfourthreehnbhkmscqxdfksg64bvpppznkh", 4, 4)]
    #[test_case("8fivenvvtrlj", 8, 5)]
    #[test_case("six3zbhvrfhsevennine", 6, 9)]
    #[test_case("427nine6chnqrssxfour", 4, 4)]
    #[test_case("threenine3five9eightrvg9", 3, 9)]
    fn test_fetch_first_and_last(input_line: &str, expected_first: u8, expected_last: u8) {
        let (first, last) = fetch_first_and_last_digits(input_line);
        assert_eq!(first, expected_first);
        assert_eq!(last, expected_last);
    }

    #[test]
    fn base_case_part_2() {
        let example_input = r"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        ";

        let expected_output = 281;

        let output = calibration_values(example_input);

        assert_eq!(output, expected_output)
    }
}