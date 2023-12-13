#![feature(array_windows)]

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    let result = summarize_reflections(INPUT);
    println!("Result: {result}");
}

fn summarize_reflections(input: &str) -> usize {
    let patterns: Vec<Vec<&str>> = input
        .trim()
        .split("\n\n")
        .map(|pat| {
            pat.lines().collect()
        })
        .collect();

    patterns
        .into_par_iter()
        .map(get_reflections)
        .map(|r| r.into_iter().map(Reflection::summary).sum::<usize>())
        .sum()
}

fn get_reflections(pattern: Vec<&str>) -> Vec<Reflection> {
    let mut reflections = Vec::new();

    let upper_bound = 0;
    let lower_bound = pattern.len();

    for (up_idx, [up, down]) in pattern.array_windows().enumerate() {
        if up == down {
            let mut upper_idx = up_idx as isize - 1;
            let mut lower_idx = up_idx + 2;

            let mut is_reflecting = true;

            while is_reflecting && upper_idx >= upper_bound as isize && lower_idx < lower_bound {
                is_reflecting = pattern[upper_idx as usize] == pattern[lower_idx];
                upper_idx -= 1;
                lower_idx += 1;
            }

            if is_reflecting {
                reflections.push(Reflection::Horizontal(up_idx + 1));
            }
        }
    }

    let left_bound = 0;
    let right_bound = pattern[0].len();

    for (left_idx, right_idx) in (left_bound..right_bound-1).zip(left_bound+1..right_bound) {
        if compare_vertically(&pattern, left_idx, right_idx, upper_bound, lower_bound) {
            let mut inner_left_idx = left_idx as isize - 1;
            let mut inner_right_idx = right_idx + 1;

            let mut is_reflecting = true;

            while is_reflecting && inner_left_idx >= left_bound as isize && inner_right_idx < right_bound {
                is_reflecting = compare_vertically(&pattern, inner_left_idx as usize, inner_right_idx, upper_bound, lower_bound);
                inner_left_idx -= 1;
                inner_right_idx += 1;
            }

            if is_reflecting {
                reflections.push(Reflection::Vertical(left_idx + 1));
            }
        }
    }

    reflections
}

fn compare_vertically(m: &[&str], left_idx: usize, right_idx: usize, upper_bound: usize, lower_bound: usize) -> bool {
    // (upper_bound..lower_bound).all(|row_idx| {
    //     m[row_idx].chars().nth(left_idx) == m[row_idx].chars().nth(right_idx)
    // })

    for row_idx in upper_bound..lower_bound {
        if m[row_idx].chars().nth(left_idx) != m[row_idx].chars().nth(right_idx) {
            return false;
        }
    }

    true
}

enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

impl Reflection {
    fn summary(self) -> usize {
        match self {
            Reflection::Horizontal(h) => h * 100,
            Reflection::Vertical(v) => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use rayon::{str::ParallelString, iter::ParallelIterator};

    use crate::summarize_reflections;

    const EXAMPLE_INPUT: &str = r"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn example_case() {
        const EXPECTED_OUTPUT: usize = 405;
        let result = summarize_reflections(EXAMPLE_INPUT);
        assert_eq!(result, EXPECTED_OUTPUT);
    }    
}