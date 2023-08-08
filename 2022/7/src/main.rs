#![feature(let_chains)]

mod fs;
mod common;
mod parsing;

use common::EntryName;
use fs::{Entry, DirRef};
use parsing::{LineType, FileListingType, CommandType};

const DIRECTORY_SIZE_LIMIT_TO_FIND: u32 = 100_000;
const TOTAL_DISK_SPACE: u32 = 70_000_000;
const REQUIRED_DISK_SPACE: u32 = 30_000_000;

fn main() {

    let input: Vec<&str> = include_str!("../input.txt").lines().collect();

    let root = DirRef::new();

    populate_entries(input, &root);

    let total_used_space = root.get_calculated_size(); // Forces directory and subdirectory size calculation
    let available_space = TOTAL_DISK_SPACE - total_used_space;

    let space_to_free = REQUIRED_DISK_SPACE - available_space;

    // root.dump();

    let dirs_up_to = find_dir_sizes_up_to(&root, DIRECTORY_SIZE_LIMIT_TO_FIND);

    // dbg!(&dirs_up_to);

    println!(
        "Sum of sizes: {}",
        dirs_up_to.iter().map(|(_, size)| size).sum::<u32>()
    ); // Part 1

    let dir_to_delete = find_minimum_size_dir(&root, space_to_free);

    if let Some(dir) = dir_to_delete {
        println!(
            "Size of the directory to delete: {:?}",
            dir.get_calculated_size()
        );
    }
    else {
        println!("No directory found to delete!");
    } // Part 2

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

fn find_minimum_size_dir(root: &DirRef, size_required: u32) -> Option<DirRef> {

    let dir = root.0.borrow();

    let mut current_minimum = if root.get_calculated_size() >= size_required {
        Some(root.clone())
    } else {
        None
    };

    for (name, entry) in dir.entries.iter() {
        if let Entry::Directory(subdir) = entry &&
            matches!(name, EntryName::Regular(..))
        {
            let mut candidate = if subdir.get_calculated_size() >= size_required {
                Some(subdir.clone())
            }
            else {
                None
            };

            let subdir_candidate = find_minimum_size_dir(subdir, size_required);

            candidate = match (&candidate, &subdir_candidate) {
                (Some(candidate_dir), Some(subdir_candidate_dir)) => {
                    if candidate_dir.get_calculated_size() < subdir_candidate_dir.get_calculated_size() {
                        candidate
                    } else {
                        subdir_candidate
                    }
                },
                (Some(..), None) => candidate,
                (None, Some(..)) => subdir_candidate,
                (None, None) => None,
            };

            current_minimum = match (&current_minimum, &candidate) {
                (Some(curr_dir), Some(candidate_dir)) => {
                    if curr_dir.get_calculated_size() < candidate_dir.get_calculated_size() {
                        current_minimum
                    } else {
                        candidate
                    }
                },
                (Some(..), None) => current_minimum,
                (None, Some(..)) => candidate,
                (None, None) => None,
            }
        }
    }

    current_minimum

}