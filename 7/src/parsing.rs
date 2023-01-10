use std::str::FromStr;

use crate::{common::{EntryName, EntrySize}, fs::{Entry, DirRef}};

#[derive(Debug)]
pub enum LineType {
    Command(CommandType),
    FileListing(FileListingType),
}

#[derive(Debug)]
pub enum FileListingType {
    File(EntrySize, EntryName),
    Directory(EntryName),
}

#[derive(Debug)]
pub enum CommandType {
    Cd(EntryName),
    Ls,
}

impl FromStr for LineType {
    type Err = LineTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let possible_command = s.parse::<CommandType>();

        match possible_command {
            Ok(command) => Ok(Self::Command(command)),
            Err(command_error) => {
                let possible_file_listing = s.parse::<FileListingType>();

                match possible_file_listing {
                    Ok(file_listing) => Ok(Self::FileListing(file_listing)),
                    Err(file_listing_error) => Err(
                        Self::Err::NeitherCommandNorLine(
                            Some(command_error),
                            Some(file_listing_error)
                        )
                    ),
                }
            }
        }
    }
}

impl FromStr for CommandType {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("$ ") {
            return Err(Self::Err::NotACommandError);
        }

        let (_, command) = s.split_once("$ ").ok_or(Self::Err::SplitError)?;

        if command.starts_with("cd") {
            let (_, directory) = command.split_once(' ').ok_or(Self::Err::SplitError)?;
            Ok(Self::Cd(directory.parse().unwrap()))
        }
        else if command.starts_with("ls") {
            Ok(Self::Ls)
        }
        else {
            Err(Self::Err::UnknownCommand(command.to_owned()))
        }
    }
}

impl FromStr for FileListingType {
    type Err = FileListingParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size_or_dir, name) = s.split_once(' ').ok_or(Self::Err::SplitError)?;

        if size_or_dir == "dir" {
            Ok(Self::Directory(name.parse().unwrap()))
        }

        else if let Ok(size) = size_or_dir.parse::<u32>() {
            Ok(Self::File(
                EntrySize::Intrisic(size),
                name.parse().unwrap()
            ))
        }

        else {
            Err(Self::Err::UnknownFileListing(s.to_owned()))
        }
    }
}

impl From<FileListingType> for (EntryName, Entry) {
    fn from(value: FileListingType) -> Self {
        match value {
            FileListingType::File(size, name) => (name, Entry::File { size }),
            FileListingType::Directory(name) => {
                let entry = Entry::Directory(
                    DirRef::new()
                );

                (name, entry)
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum CommandParseError {
    SplitError,
    UnknownCommand(String),
    NotACommandError,
}

#[derive(Debug, Clone)]
pub enum FileListingParseError {
    SplitError,
    UnknownFileListing(String),
}

#[derive(Debug, Clone)]
pub enum LineTypeParseError {
    NeitherCommandNorLine(
        Option<CommandParseError>,
        Option<FileListingParseError>,
    )
}