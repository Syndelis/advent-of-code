#![feature(let_chains)]

use std::collections::HashSet;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use strum::FromRepr;

fn main() -> Result<(), MapError> {
    const INPUT: &str = include_str!("../input.txt");
    let result = get_points_enclosed_by_maze(INPUT)?;
    println!("Result: {result}");
    Ok(())
}

fn find_furthest_position(input: &str) -> Result<usize, MapError> {
    let (origin, map) = parse_map(input)
        .map_err(MapError::TileParseError)?;

    let loop_length = get_maze_path(origin, &map)
        .ok_or(MapError::NoLoopFound)?
        .1
        .len();

    Ok(loop_length / 2)
}

fn get_points_enclosed_by_maze(input: &str) -> Result<usize, MapError> {
    let (origin, map) = parse_map(input)
        .map_err(MapError::TileParseError)?;

    let (start_tile_replacement, maze_path) = get_maze_path(origin, &map)
        .ok_or(MapError::NoLoopFound)?;

    let y_limit = map.len();
    let x_limit = map[0].len();

    let mut count = 0;

    let banned_tiles: HashSet<Tile> = [Tile::NorthEastPipe, Tile::SouthWestPipe].into();

    for (y, v) in map.iter().enumerate() {
        for (x, _) in v.iter().enumerate() {
            let mut pos = (x, y);

            if maze_path.contains(&pos) {
                continue;
            }

            let mut inside = false;

            while pos.0 < x_limit && pos.1 < y_limit {
                let tile = &map[pos.1][pos.0];
                let tile = if matches!(tile, Tile::StartingPosition) {
                    &start_tile_replacement
                } else {
                    tile
                };

                if maze_path.contains(&pos) && !banned_tiles.contains(tile) {
                    inside = !inside;
                }

                pos = (pos.0 + 1, pos.1 + 1);
            }

            if inside {
                count += 1;
            }
        }
    }

    Ok(count)
}

fn get_maze_path(origin: Coordinate, map: &Vec<Vec<Tile>>) -> Option<(Tile, HashSet<Coordinate>)> {
    let y_limit = map.len() - 1;
    let x_limit = map[0].len() - 1;
    
    get_adjacent_directions(origin, (x_limit, y_limit))
        .into_par_iter()
        .filter_map(move |starting_direction| {
            let mut pos = origin;
            let mut step_state = StepState::Continue(starting_direction);
            let mut positions = HashSet::new();
            let mut last_dir = starting_direction;

            while let StepState::Continue(dir) = step_state {
                positions.insert(pos);
                pos = dir + pos;
                let tile = &map[pos.1][pos.0];
                step_state = dir.step(tile);
                last_dir = dir;
            }

            if let StepState::ReachedGoal = step_state {
                Some((Tile::next(last_dir, starting_direction), positions))
            } else {
                None
            }
        })
        .find_first(|_| true)
}

fn get_adjacent_directions((x, y): Coordinate, (x_limit, y_limit): Coordinate) -> Vec<Direction> {
    use Direction::*;
    let mut v = Vec::with_capacity(4);

    if x <= x_limit {
        v.push(East);
    }

    if x > 0 {
        v.push(West);
    }

    if y <= y_limit {
        v.push(South);
    }

    if y > 0 {
        v.push(North)
    }

    v
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
#[derive(Debug, FromRepr, Clone, Copy, PartialEq, Eq, Hash)]
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

impl Tile {
    fn connects_horizontally_to(&self, right: &Self) -> bool {
        use Tile::*;
        let left = self;

        match (left, right) {
            (Ground, _)
            | (_, Ground) => false,

            (StartingPosition, _)
            | (_, StartingPosition) => panic!("Should not be called with StartingPosition"),

            (VerticalPipe, _)
            | (_, VerticalPipe) => false,

            (SouthEastPipe, NorthWestPipe)
            | (SouthEastPipe, SouthWestPipe)
            | (NorthEastPipe, NorthWestPipe)
            | (NorthEastPipe, SouthWestPipe) => true,

            (HorizontalPipe, HorizontalPipe)
            | (SouthEastPipe, HorizontalPipe)
            | (NorthEastPipe, HorizontalPipe)
            | (HorizontalPipe, SouthWestPipe)
            | (HorizontalPipe, NorthWestPipe) => true,

            (SouthWestPipe, _)
            | (NorthWestPipe, _) => false,

            (SouthEastPipe, SouthEastPipe)
            | (SouthEastPipe, NorthEastPipe)
            | (NorthEastPipe, SouthEastPipe)
            | (NorthEastPipe, NorthEastPipe)
            | (HorizontalPipe, SouthEastPipe)
            | (HorizontalPipe, NorthEastPipe) => false,
            
        }
    }

    fn connects_vertically_to(&self, down: &Self) -> bool {
        use Tile::*;
        let up = self;

        match (up, down) {
            (Ground, _)
            | (_, Ground) => false,

            (StartingPosition, _)
            | (_, StartingPosition) => panic!("Should not be called with StartingPosition"),

            (HorizontalPipe, _)
            | (_, HorizontalPipe) => false,

            (SouthEastPipe, NorthEastPipe)
            | (SouthEastPipe, NorthWestPipe)
            | (SouthWestPipe, NorthEastPipe)
            | (SouthWestPipe, NorthWestPipe) => true,

            (VerticalPipe, VerticalPipe)
            | (VerticalPipe, NorthEastPipe)
            | (VerticalPipe, NorthWestPipe)
            | (SouthEastPipe, VerticalPipe)
            | (SouthWestPipe, VerticalPipe) => true,

            (NorthEastPipe, _)
            | (NorthWestPipe, _) => false,

            (VerticalPipe, SouthWestPipe)
            | (VerticalPipe, SouthEastPipe)
            | (SouthWestPipe, SouthWestPipe)
            | (SouthWestPipe, SouthEastPipe)
            | (SouthEastPipe, SouthWestPipe)
            | (SouthEastPipe, SouthEastPipe) => false,
            
        }
    }

    fn next(coming_from: Direction, exiting_to: Direction) -> Tile {
        use Direction::*;
        use Tile::*;

        match (coming_from, exiting_to) {
            (West, East)
            | (East, West)
            | (West, West)
            | (East, East) => HorizontalPipe,

            (North, South)
            | (South, North)
            | (North, North)
            | (South, South) => VerticalPipe,

            (North, West)
            | (East, South) => SouthWestPipe,

            (North, East)
            | (West, South) => SouthEastPipe,

            (South, West)
            | (East, North) => NorthWestPipe,

            (South, East)
            | (West, North) => NorthEastPipe,
        }
    }
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

    use crate::{find_furthest_position, get_points_enclosed_by_maze};

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
    fn example_cases_part_1(maze: &str, expected_furthest_position: usize) {
        let result = find_furthest_position(maze).unwrap();
        assert_eq!(result, expected_furthest_position);
    }

    #[test_case(
        r"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
        ",
        4
    )]
    #[test_case(
        r"
        ..........
        .S------7.
        .|F----7|.
        .||....||.
        .||....||.
        .|L-7F-J|.
        .|..||..|.
        .L--JL--J.
        ..........
        ",
        4
    )]
    #[test_case(
        r"
        ...........
        .S-------7.
        .|F-----7|.
        .||-----||.
        .|||||||||.
        .|L-7|F-J|.
        .|L7|||--|.
        .L--J|L--J.
        ...........
        ",
        4 ; "Same example but with additional pipes"
    )]
    #[test_case(
        r"
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
        ",
        8
    )]
    fn example_cases_part_2(maze: &str, expected_enclosed_count: usize) {
        let result = get_points_enclosed_by_maze(maze).unwrap();
        assert_eq!(result, expected_enclosed_count);
    }
}