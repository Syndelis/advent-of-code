use std::{str::FromStr, convert::Infallible};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntryName {
    Root,
    UpDir,
    Regular(String),
}

#[derive(Debug, Clone)]
pub enum EntrySize {
    Intrisic(u32),
    Calculated {
        size: u32,
        includes_indirect_sizes: bool,
    }
}

impl EntrySize {
    pub fn get_size(&self) -> u32 {
        match self {
            Self::Intrisic(size) => *size,
            Self::Calculated { size, .. } => *size,
        }
    }
}

impl FromStr for EntryName {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "/" => Self::Root,
            ".." => Self::UpDir,
            _ => Self::Regular(s.to_owned())
        })     
    }
}

impl EntryName {
    pub fn as_str(&self) -> &str {
        match self {
            EntryName::Root => "/",
            EntryName::UpDir => "..",
            EntryName::Regular(name) => name.as_str(),
        }
    }
}
