use std::{fs::File, io::Read};

fn main() {

    let mut input = String::new();
    {
        let mut input_file = File::open("input.txt").expect("Could not open input file");
        input_file.read_to_string(&mut input).expect("Could not read input file");
    }

    let input = input.split("\n\n").collect::<Vec<&str>>();
    let mut elves_totals: Vec<i32> = input.into_iter().map(|elf_inventory_str| {
        elf_inventory_str.split('\n').map(|line| line.parse::<i32>().unwrap_or(0)).sum()
    }).collect();

    // Part 1
    println!("Largest inventory is {}", elves_totals.iter().max().unwrap());

    // Part 2
    elves_totals.sort_by_key(|total| -total);

    let top_three_elves = &elves_totals[0..3];

    println!("Top three elves have {} total inventory", top_three_elves.iter().sum::<i32>());

}
