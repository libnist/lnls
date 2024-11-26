use std::{error::Error, fs::File};
use std::path::Path;
use std::fs;

use chrono::{DateTime, Local};

pub struct Config {
    path: String,
    show_hidden: bool

}

#[derive(Debug)]
enum Entry {
    FileEntry(FileEntry),
    DirectoryEntry(Vec<FileEntry>)
}

#[derive(Debug)]
struct FileEntry {
    file_name: String,
    size: u64,
    modified_on: String,
}

impl FileEntry {

    fn new(path: &Path) -> Result<FileEntry, Box<dyn Error>> {
        if let Some(file_name) = path.file_name() {
            let modified: DateTime<Local> = DateTime::from(path.metadata()?.modified()?);
            let file_entry = FileEntry {
                file_name: file_name.to_str().unwrap().to_string(),
                size: path.metadata()?.len(),
                modified_on: modified.format("%_d %b %H:%M").to_string(),
            };
            return Ok(file_entry)
        } else {
            return Err(format!("Invalid entry: {:?}", path))?
        }
    }

    fn print(&self) {
        println!("{:>5}, {}, {}", self.size, self.modified_on, self.file_name);
    }
}

impl Entry {
    fn new(path: &Path) -> Result<Entry, Box<dyn Error>> {
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

    fn print(&self) {
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


/// The point where the program starts looking up for direcotries and files.
/// 
/// # Example
/// 
/// ```
/// 
/// ```
pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    if path.is_dir() {
        directory_list(path)?
    } else if path.is_file() {
        Entry::new(path)?.print();
    }
    Ok(())
}

fn directory_list(dir: &Path) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let file_name = entry
                                    .file_name()
                                    .into_string()
                                    .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
        println!("{}", file_name);
    }

    Ok(())
}