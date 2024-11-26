//! This module is to help get the file or files
//! inside a given path.


use std::error::Error;
use std::path::Path;
use std::fs;

use chrono::{DateTime, Local};

/// The `Entry` enum has two variation:
///  * FileEntry(file_entry): in case the given argument is a file.
///  * DirectoryEntry(Vec<file_entry>): in case the given argument is a directory.
/// 
/// # Example
/// ```
/// let path = Path::new(".");
/// let entry = Entry::new(path);
/// ```
#[derive(Debug)]
pub enum Entry {
    FileEntry(FileEntry),
    DirectoryEntry(Vec<FileEntry>)
}

impl Entry {
    /// The new function returns a `Result<Entry, Box<dyn Error>>`
    ///  given a &Path.
    pub fn new(path: &Path) -> Result<Entry, Box<dyn Error>> {
        if path.is_dir() {
            let mut file_entries: Vec<FileEntry> = Vec::new();
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                file_entries.push(FileEntry::new(entry.path().as_path())?);
            }
            return Ok(Entry::DirectoryEntry(file_entries));
        } else if path.is_file() {
            return Ok(Entry::FileEntry(FileEntry::new(path)?));
        } else {
            Err(format!("The given input is not a file or directory: {:?}", path))?
        }
    }

    // The print method prints each file/directory in a line.
    pub fn print(&self) {
        match self {
            Entry::FileEntry(file_entry) => file_entry.print(),
            Entry::DirectoryEntry(file_entries) => {
                for file_entry in file_entries {
                    file_entry.print();
                }
            }
            
        }
    }
}

/// The FileEntry struct is to save all the information we need from a file
/// 
/// # Example
/// ```
/// let path = Path::new(".");
/// let file_entry = FileEntry::new(path);
/// ```
#[derive(Debug)]
pub(crate) struct FileEntry {
    file_name: String,
    size: u64,
    modified_on: String,
}

impl FileEntry {
    /// The new function accepts an `&Path` and returns `Result<FileEntry, Box<dyn Error>>`.
    fn new(path: &Path) -> Result<FileEntry, Box<dyn Error>> {
        if let Some(file_name) = path.file_name() {
            let metadata = path.metadata()?;
            let modified: DateTime<Local> = DateTime::from(metadata.modified()?);
            let file_entry = FileEntry {
                file_name: file_name.to_str().unwrap().to_string(),
                size: metadata.len(),
                modified_on: modified.format("%_d %b %H:%M").to_string(),
            };
            return Ok(file_entry)
        } else {
            return Err(format!("Invalid entry: {:?}", path))?
        }
    }
    /// The print method prints all the information we need from the file.
    fn print(&self) {
        println!("{:>5}, {}, {}", self.size, self.modified_on, self.file_name);
    }
}