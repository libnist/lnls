use std::error::Error;
use std::path::Path;
use std::fs;

pub struct Config {
    paths: Vec<String>,

}



pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    if path.is_dir() {
        directory_list(path)?
    } else if path.is_file() {
        file_list(path)?
    }
    Ok(())
}

fn directory_list(dir: &Path) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(dir)? {
        let file_name = entry?
                                    .file_name()
                                    .into_string()
                                    .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
        println!("{}", file_name);
    }

    Ok(())
}

fn file_list(file: &Path) -> Result<(), Box<dyn Error>> {
    if let Some(file_name) = file.file_name() {
        println!("{}", file.parent().unwrap().to_str().unwrap());
        println!("{}", file_name.to_str().unwrap());
    } else {
        Err(format!("Invalid entry: {:?}", file))?
    }

    Ok(())
}