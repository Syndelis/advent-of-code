use rayon::iter::{IntoParallelIterator, ParallelIterator};
use strum::FromRepr;

fn main() -> Result<(), MapError> {
    const INPUT: &str = include_str!("../input.txt");
    let result = find_furthest_position(INPUT)?;
    println!("Result: {result}");
    Ok(())
}

fn find_furthest_position(input: &str) -> Result<u32, MapError> {
    let (origin, map) = parse_map(input)
        .map_err(MapError::TileParseError)?;

    let limit = map.len() - 1;
    
    let start_at = {
        use Direction::*;
        let mut v = Vec::with_capacity(4);

        let (x, y) = origin;

        if x <= limit {
            v.push(East);
        }

        if x > 0 {
            v.push(West);
        }

        if y <= limit {
            v.push(South);
        }

        if y > 0 {
            v.push(North)
        }

        v
    };

    let loop_length = start_at
        .into_par_iter()
        .filter_map(|starting_direction| {
            let mut pos = origin;
            let mut step_state = StepState::Continue(starting_direction);
            let mut steps_taken = 0;

            while let StepState::Continue(dir) = step_state {
                pos = dir + pos;
                let tile = &map[pos.1][pos.0];
                step_state = dir.step(tile);
                steps_taken += 1
            }

            if let StepState::ReachedGoal = step_state {
                Some(steps_taken)
            } else {
                None
            }
        })
        .max()
        .ok_or(MapError::NoLoopFound)?;

    Ok(loop_length / 2)

}

#[derive(Debug)]
enum MapError {
    TileParseError(InvalidTile),
    NoLoopFound,
}

type Coordinate = (usize, usize);
type Map = Vec<Vec<Tile>>;

fn parse_map(input: &str) -> Result<(Coordinate, Map), InvalidTile> {
    let mut starting_pos = (0, 0);

    let map = input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line
                .trim()
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    let t = Tile::from_repr(c as u64)
                            .ok_or(InvalidTile(c))?;

                    if let Tile::StartingPosition = t {
                        starting_pos = (x, y);
                    }

                    Ok(t)
                })
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()?;

    Ok((starting_pos, map))
}

#[derive(Debug)]
struct InvalidTile(char);

#[repr(u64)]
#[derive(Debug, FromRepr)]
enum Tile {
    Ground = '.' as u64,
    StartingPosition = 'S' as u64,
    VerticalPipe = '|' as u64,
    HorizontalPipe = '-' as u64,
    SouthWestPipe = '7' as u64,
    NorthWestPipe = 'J' as u64,
    SouthEastPipe = 'F' as u64,
    NorthEastPipe = 'L' as u64,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl std::ops::Add<Coordinate> for Direction {
    type Output = Coordinate;

    fn add(self, (x, y): Coordinate) -> Self::Output {
        match self {
            Direction::North => (x, y - 1),
            Direction::West => (x - 1, y),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
        }
    }
}

enum StepState {
    Continue(Direction),
    ReachedGoal,
    CantMove,
}

impl Direction {
    fn step(self, tile: &Tile) -> StepState {
        use Direction::*;
        use Tile::*;
        use StepState::*;

        match (self, tile) {
            (_, Ground) => CantMove,
            (_, StartingPosition) => ReachedGoal,

            (North, VerticalPipe) => Continue(North),
            (North, SouthWestPipe) => Continue(West),
            (North, SouthEastPipe) => Continue(East),

            (South, VerticalPipe) => Continue(South),
            (South, NorthWestPipe) => Continue(West),
            (South, NorthEastPipe) => Continue(East),

            (East, HorizontalPipe) => Continue(East),
            (East, SouthWestPipe) => Continue(South),
            (East, NorthWestPipe) => Continue(North),

            (West, HorizontalPipe) => Continue(West),
            (West, SouthEastPipe) => Continue(South),
            (West, NorthEastPipe) => Continue(North),

            _ => CantMove,
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::find_furthest_position;

    #[test_case(
        r"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
        ",
        4 ; "Simple loop"
    )]
    #[test_case(
        r"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
        ",
        8 ; "Complex loop"
    )]
    fn example_cases(maze: &str, expected_furthest_position: u32) {
        let result = find_furthest_position(maze).unwrap();
        assert_eq!(result, expected_furthest_position);
    }
}