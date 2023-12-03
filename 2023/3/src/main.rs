use std::ops::Range;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    let result = gear_ratios(INPUT);
    println!("Result: {result}");
}

fn gear_ratios(input: &str) -> u32 {
    let (numbers, symbols) = parse_numbers_and_symbols(input);
    get_numbers_adjacent_to_symbols(numbers, symbols).sum()
}

fn parse_numbers_and_symbols(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    for (row, line) in input.lines().enumerate() {
        let mut col_span_begin = None;
        let line = line.trim();

        for (col, c) in line.chars().enumerate() {
            match col_span_begin {
                Some(col_begin) if !c.is_numeric() => {
                    numbers.push(Number::from_span(line, row, col_begin..col));
                    col_span_begin = None;
                },
                None if c.is_numeric() => {
                    col_span_begin = Some(col);
                }
                _ => {}
            }

            if c.is_symbol() {
                symbols.push(Symbol {
                    pos: Position {
                        col,
                        row,
                    }
                });
            }
        }

        if let Some(col_begin) = col_span_begin {
            numbers.push(Number::from_span(line, row, col_begin..line.len()));
        }
    }

    (numbers, symbols)
}

fn get_numbers_adjacent_to_symbols(numbers: Vec<Number>, symbols: Vec<Symbol>) -> impl Iterator<Item = u32> {
    numbers
        .into_iter()
        .filter_map(move |number| {
            symbols
                .iter()
                .any(|symbol| number.pos.is_adjacent(&symbol.pos))
                .then_some(number.val)
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Number {
    val: u32,
    pos: Position<Range<usize>>,
}

impl Number {
    fn from_span(line: &str, row: usize, col: Range<usize>) -> Self {
        let pos = Position {
            row,
            col,
        };

        let val = line[pos.col.clone()].parse().unwrap();

        Self {
            val,
            pos,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Symbol {
    pos: Position<usize>
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Position<T> {
    row: usize,
    col: T,
}

impl Position<Range<usize>> {
    fn is_adjacent(&self, to: &Position<usize>) -> bool {
        let row_span = (to.row-1)..=(to.row+1);
        row_span.contains(&self.row) && self.col.extend(1).contains(&(to.col as isize))
    }
}

trait RangeExtension {
    fn extend(&self, by: isize) -> Range<isize>;
}

impl RangeExtension for Range<usize> {
    fn extend(&self, by: isize) -> Range<isize> {
        let start = self.start as isize;
        let end = self.end as isize;
        (start-by)..(end+by)
    }
}

trait IsSymbol {
    fn is_symbol(self) -> bool;
}

impl IsSymbol for char {
    fn is_symbol(self) -> bool {
        !self.is_numeric() && self != '.'
    }
}

#[cfg(test)]
mod tests {
    use crate::{gear_ratios, Number, Symbol, Position, parse_numbers_and_symbols};
    use test_case::test_case;

    #[test]
    fn example_case() {
        const INPUT: &str = r"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        ";

        const EXPECTED_OUTPUT: u32 = 4361;

        let result = gear_ratios(INPUT);

        assert_eq!(result, EXPECTED_OUTPUT);
    }

    #[test_case(
        "467..114..",
        vec![
            Number {
                val: 467,
                pos: Position {
                    row: 0,
                    col: 0..3,
                }
            },
            Number {
                val: 114,
                pos: Position {
                    row: 0,
                    col: 5..8,
                }
            }
        ],
        vec![]
    )]
    #[test_case(
        "\n...*......",
        vec![],
        vec![
            Symbol {
                pos: Position {
                    row: 1,
                    col: 3,
                }
            }
        ]
    )]
    #[test_case(
        "\n\n\n\n617*......",
        vec![
            Number {
                val: 617,
                pos: Position {
                    row: 4,
                    col: 0..3,
                }
            }
        ],
        vec![
            Symbol {
                pos: Position {
                    row: 4,
                    col: 3,
                }
            }
        ]
    )]
    #[test_case(
        "123.456",
        vec![
            Number {
                val: 123,
                pos: Position {
                    row: 0,
                    col: 0..3,
                }
            },
            Number {
                val: 456,
                pos: Position {
                    row: 0,
                    col: 4..7,
                }
            }
        ],
        vec![]
    )]
    fn test_parse_numbers_and_symbols(input: &str, expected_numbers: Vec<Number>, expected_symbols: Vec<Symbol>) {
        let (numbers, symbols) = parse_numbers_and_symbols(input);
        assert_eq!(numbers, expected_numbers);
        assert_eq!(symbols, expected_symbols);
    }
}