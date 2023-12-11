use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    let result = calculate_galaxy_pairs_distances(INPUT);
    println!("Result: {result}");
}

fn calculate_galaxy_pairs_distances(input: &str) -> usize {
    let (galaxies, expanded_rows, expanded_cols) = parse_galaxies(input);

    let mut distance_sum = 0;

    for (idx, galaxy_a) in galaxies.iter().enumerate() {
        for galaxy_b in &galaxies[idx+1..] {
            let distance = distance_between_galaxies(
                galaxy_a, galaxy_b,
                &expanded_rows, &expanded_cols,
            );

            distance_sum += distance;
        }
    }

    distance_sum
}

fn distance_between_galaxies(galaxy_a: &Coord, galaxy_b: &Coord, expanded_cols: &HashSet<usize>, expanded_rows: &HashSet<usize>) -> usize {
    let (min_x, max_x) = min_max(galaxy_a.x, galaxy_b.x);
    let (min_y, max_y) = min_max(galaxy_a.y, galaxy_b.y);

    (min_x+1..=max_x)
        .map(|x| {
            if expanded_cols.contains(&x) {
                2
            } else {
                1
            }
        })
        .chain(
            (min_y+1..=max_y)
                .map(|y| {
                    if expanded_rows.contains(&y) {
                        2
                    } else {
                        1
                    }       
                })
        )
        .sum()
}

const GALAXY: char = '#';

fn parse_galaxies(input: &str) -> (Vec<Coord>, HashSet<usize>, HashSet<usize>) {

    let lines = input
        .trim()
        .lines()
        .filter(|line| !line.is_empty());

    let mut cols_with_galaxies = HashSet::new();

    let mut galaxies = Vec::new();

    let mut rows = 0;
    let mut cols = 0;

    let rows_with_galaxies: HashSet<usize> = lines
        .enumerate()
        .filter_map(|(y, line)| {
            cols = cols.max(y);
            line
                .trim()
                .char_indices()
                .filter(|(_, c)| c == &GALAXY)
                .fold(None, |_, (x, _)| {
                    rows = rows.max(x);
                    cols_with_galaxies.insert(x);
                    galaxies.push(Coord { x, y });
                    Some(y)
                })
        })
        .collect();

    let expanded_rows = HashSet::from_iter(0..rows)
        .difference(&rows_with_galaxies)
        .copied()
        .collect();

    let expanded_cols = HashSet::from_iter(0..cols)
        .difference(&cols_with_galaxies)
        .copied()
        .collect();

    (galaxies, expanded_cols, expanded_rows)
}

fn min_max<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use test_case::test_case;

    use crate::{Coord, parse_galaxies, calculate_galaxy_pairs_distances, distance_between_galaxies};


    const EXAMPLE_INPUT: &str = r"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....    
    ";

    #[test]
    fn test_parse_galaxies() {
        let expected_galaxies = [
            Coord { x: 3, y: 0 },
            Coord { x: 7, y: 1 },
            Coord { x: 0, y: 2 },
            Coord { x: 6, y: 4 },
            Coord { x: 1, y: 5 },
            Coord { x: 9, y: 6 },
            Coord { x: 7, y: 8 },
            Coord { x: 0, y: 9 },
            Coord { x: 4, y: 9 },
        ];

        let expected_expanded_rows: HashSet<_> = [
            3, 7
        ].into();

        let expected_expanded_cols: HashSet<_> = [
            2, 5, 8
        ].into();

        let (galaxies, expanded_cols, expanded_rows) = parse_galaxies(EXAMPLE_INPUT);

        assert_eq!(&galaxies, &expected_galaxies);
        assert_eq!(expanded_rows, expected_expanded_rows);
        assert_eq!(expanded_cols, expected_expanded_cols);
    }

    #[test_case(5, 9, 9)]
    #[test_case(1, 7, 15)]
    #[test_case(3, 6, 17)]
    #[test_case(8, 9, 5)]
    fn test_distance_between_galaxies(idx_a: usize, idx_b: usize, expected_distance: usize) {
        let (galaxies, expanded_cols, expanded_rows) = parse_galaxies(EXAMPLE_INPUT);

        let galaxy_a = &galaxies[idx_a-1];
        let galaxy_b = &galaxies[idx_b-1];

        let distance = distance_between_galaxies(galaxy_a, galaxy_b, &expanded_cols, &expanded_rows);

        assert_eq!(distance, expected_distance);
    }

    #[test]
    fn example_case() {
        const EXPECTED_OUTPUT: usize = 374;
        let result = calculate_galaxy_pairs_distances(EXAMPLE_INPUT);
        assert_eq!(result, EXPECTED_OUTPUT);
    }

    #[test]
    fn t() {
        const INPUT: &str = r"
            ##
            .#
        ";

        const EXPECTED_OUTPUT: usize = 4;

        let result = calculate_galaxy_pairs_distances(INPUT);
        assert_eq!(result, EXPECTED_OUTPUT);
    }
}