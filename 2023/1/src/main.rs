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
            let mut digits = line
                .chars()
                .filter(|c| c.is_numeric());
            
            let first_digit = digits.next().unwrap_or('0');
            let last_digit = digits.last();

            let first_digit = first_digit as u8 - b'0';

            (match last_digit {
                // The last digit IS the first digit!
                None => first_digit * 11,
                Some(last_digit) => first_digit * 10 + last_digit as u8 - b'0',
            }) as u64
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::calibration_values;

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

        assert_eq!(output, expected_output)
    }
}