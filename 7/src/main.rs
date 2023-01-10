#![feature(let_chains)]

mod fs;
mod common;
mod parsing;

use common::EntryName;
use fs::{Entry, DirRef};
use parsing::{LineType, FileListingType, CommandType};

const DIRECTORY_SIZE_LIMIT_TO_FIND: u32 = 100_000;

fn main() {

    let input: Vec<&str> = include_str!("../input.txt").lines().collect();

    let root = DirRef::new();

    populate_entries(input, &root);

    root.get_calculated_size(); // Forces directory and subdirectory size calculation

    // root.dump();

    let dirs_up_to = find_dir_sizes_up_to(&root, DIRECTORY_SIZE_LIMIT_TO_FIND);

    // dbg!(&dirs_up_to);

    println!(
        "Sum of sizes: {}",
        dirs_up_to.iter().map(|(_, size)| size).sum::<u32>()
    ); // Part 1

}

fn populate_entries(input: Vec<&str>, root: &DirRef) {

    let mut visitor = root.clone();

    for line in input {
        let line_type: LineType = line.parse().unwrap();

        match line_type {
            LineType::Command(command) => {
                if let CommandType::Cd(directory_name) = command {
                    match directory_name {
                        EntryName::Root => visitor = root.clone(),
                        EntryName::Regular(..) | EntryName::UpDir => visitor = visitor.get_subdir(directory_name),
                    }
                }
            },
            LineType::FileListing(file_listing) => match file_listing {
                FileListingType::File(size, name) => visitor.new_file(name, size),
                FileListingType::Directory(name) => visitor.new_subdir(name),
            }
        };

    }
}

fn find_dir_sizes_up_to(root: &DirRef, limit: u32) -> Vec<(String, u32)> {

    let mut dirs_and_sizes = Vec::new();

    let dir = root.0.borrow();

    for (name, entry) in dir.entries.iter() {
        if let Entry::Directory(subdir) = entry &&
            let EntryName::Regular(name) = name
        {
            if subdir.get_calculated_size() <= limit {
                dirs_and_sizes.push((name.clone(), subdir.get_calculated_size()));
            }
            dirs_and_sizes.extend(find_dir_sizes_up_to(subdir, limit))
        }
    }

    dirs_and_sizes

}