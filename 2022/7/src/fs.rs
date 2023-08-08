use std::{collections::HashMap, cell::RefCell, rc::Rc};

use crate::common::{EntrySize, EntryName};

#[derive(Debug)]
pub enum Entry {
    Directory(DirRef),
    File {
        size: EntrySize
    },
}

#[derive(Debug)]
pub struct Directory {
    pub entries: HashMap<EntryName, Entry>,
    pub size: EntrySize,
}

#[derive(Debug)]
pub struct DirRef(pub Rc<RefCell<Directory>>);

impl Clone for DirRef {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl DirRef {
    pub fn new() -> Self {
        Self(
            Rc::new(RefCell::new(
                Directory {
                    entries: HashMap::new(),
                    size: EntrySize::Calculated {
                        size: 0,
                        includes_indirect_sizes: false
                    }
                }
            ))
        )
    }

    pub fn new_subdir(&self, name: EntryName) {
        if let EntryName::Regular(name) = name {
            let new_subdir = DirRef::default();

            new_subdir.0.borrow_mut().entries.insert(
                EntryName::UpDir,
                Entry::Directory(DirRef(Rc::clone(&self.0)))
            );

            self.0.borrow_mut().entries.insert(
                EntryName::Regular(name),
                Entry::Directory(DirRef(Rc::clone(&new_subdir.0)))
            );
        }

        else {
            panic!("Cannot add root ( / ) to another directory")
        }
    }

    pub fn new_file(&self, name: EntryName, size: EntrySize) {
        if let EntryName::Regular(name) = name {
            self.0.borrow_mut().entries.insert(
                EntryName::Regular(name),
                Entry::File { size }
            );
        }

        else {
            panic!("The name `/` cannot be attributed to a file");
        }
    }

    pub fn get_subdir(&self, name: EntryName) -> DirRef {

        match name {
            EntryName::Root => unreachable!("This case is already checked by the caller"),
            _ => {
                if let Some(Entry::Directory(dir)) = self.0.borrow().entries.get(&name) {
                    dir.clone()
                }
                else {
                    panic!("Directory `{}` does not exist!", name.as_str());
                }
            }
        }
    }

    pub fn get_calculated_size(&self) -> u32 {

        if let EntrySize::Calculated { size, includes_indirect_sizes } = self.0.borrow().size {
            if includes_indirect_sizes {
                return size;
            }
        }

        let mut dir = self.0.borrow_mut();

        let mut size = 0;

        for (name, entry) in dir.entries.iter() {
            match entry {
                Entry::Directory(subdir) => {
                    if matches!(name, EntryName::Regular( .. )) {
                        let subdir = DirRef(Rc::clone(&subdir.0));
                        size += subdir.get_calculated_size();
                    }
                },
                Entry::File { size: file_size } => {
                    size += file_size.get_size();
                }
            }
        }

        dir.size = EntrySize::Calculated {
            size,
            includes_indirect_sizes: true
        };

        size
    }

    pub fn dump(&self) {
        for (name, entry) in self.0.borrow().entries.iter() {
            match entry {
                Entry::Directory(dir) => {
                    match name {
                        EntryName::Root => unreachable!("Root cannot be a subdir"),
                        EntryName::UpDir => println!("dir .. enddir"),
                        EntryName::Regular(name) => {
                            println!("dir {name} {:?}", dir.get_calculated_size());
                            dir.dump();
                            println!("enddir {name}");
                        }
                    }
                },
                Entry::File { size } => {
                    println!("File: {name:?} {size:?}");
                }
            }
        }
    }
}

impl Default for DirRef {
    fn default() -> Self {
        Self::new()
    }
}