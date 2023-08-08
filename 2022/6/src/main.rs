use std::collections::HashSet;

const PACKET_START_MARKER_LENGTH: usize = 4; // Part 1
const MESSAGE_START_MARKER_LENGTH: usize = 14; // Part 2

fn main() {

    let input = include_str!("../input.txt");
    let input_length = input.len();

    for idx in 0..input_length - MESSAGE_START_MARKER_LENGTH {
        let idx_range = idx..idx + MESSAGE_START_MARKER_LENGTH;

        let char_range = &input[idx_range];

        let unique_characters: HashSet<char> = HashSet::from_iter(char_range.chars());

        if unique_characters.len() == MESSAGE_START_MARKER_LENGTH {
            // Part 2
            println!("{}: {char_range}", idx + MESSAGE_START_MARKER_LENGTH);
            break;
        }
    }

}
