use std::collections::HashSet;

const PACKET_START_LENGTH: usize = 4;

fn main() {

    let input = include_str!("../input.txt");
    let input_length = input.len();

    for idx in 0..input_length - PACKET_START_LENGTH {
        let idx_range = idx..idx + PACKET_START_LENGTH;

        let char_range = &input[idx_range];

        let unique_characters: HashSet<char> = HashSet::from_iter(char_range.chars());

        if unique_characters.len() == PACKET_START_LENGTH {
            // Part 1
            println!("{}: {char_range}", idx + PACKET_START_LENGTH);
            break;
        }
    }

}
