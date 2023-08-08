#![feature(array_chunks)]

use std::collections::{HashSet, hash_set::Intersection};

fn priority_of(c: &char) -> u32 {

    if !c.is_ascii() {
        panic!("Invalid item {c}")
    }

    let c_ascii = *c as u8;

    if ('a'..='z').contains(c) {
        (c_ascii - b'a' + 1) as u32
    }
    else if ('A'..='Z').contains(c) {
        (c_ascii - b'A' + 27) as u32
    }
    else {
        panic!("Invalid item {c}")
    }
}

fn main() {
    
    let input: Vec<&str> = include_str!("../input.txt").split_terminator('\n').collect();

    // Part 1
    let total_shared_priorities = input.clone().into_iter().map(|sack: &str| {
        let compartment_size = sack.len() / 2;

        let first_compartment: HashSet<char> = sack[0..compartment_size].chars().collect();
        let second_compartment: HashSet<char> = sack[compartment_size..].chars().collect();

        let shared_items = first_compartment.intersection(&second_compartment);

        shared_items.map(priority_of).sum::<u32>()
    });

    println!("Total shared priorities: {}", total_shared_priorities.sum::<u32>());

    // Part 2
    let elf_groups = input.array_chunks::<3>();

    let badges = elf_groups.map(|group| {
        let group: Vec<HashSet<char>> = group.iter().map(|s| s.chars().collect()).collect();
        group.iter().fold(group[0].clone(), |acc, set| acc.intersection(set).copied().collect())
    });

    let badges_priority_sum = badges.fold(
        0, |acc, badge| acc + priority_of(badge.iter().next().unwrap())
    );

    println!("Badges priority sum: {badges_priority_sum}");

}
