use std::{str::FromStr, ops::RangeInclusive};

struct CleaningSector(RangeInclusive<i32>);

impl FromStr for CleaningSector {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('-');
        let start = iter.next().unwrap().parse().unwrap();
        let end = iter.next().unwrap().parse().unwrap();
        Ok(CleaningSector(start..=end))
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let sector_pairs: Vec<(CleaningSector, CleaningSector)> = input.lines()
        .map(|line| {
            let mut iter = line.split(',');
            let sector1 = iter.next().unwrap().parse().unwrap();
            let sector2 = iter.next().unwrap().parse().unwrap();
            (sector1, sector2)
        })
        .collect();
    
    // Part 1

    let mut overlaps = 0;

    for (sector1, sector2) in sector_pairs {
        if (
              sector1.0.start() <= sector2.0.start()
            && sector1.0.end() >= sector2.0.end()
        ) || (
            sector2.0.start() <= sector1.0.start()
            && sector2.0.end() >= sector1.0.end()
        ) {
            overlaps += 1;
        }
    }

    println!("Part 1: {overlaps}");
}
