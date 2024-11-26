mod file_entry;

use std::error::Error;
use std::path::Path;

use self::file_entry::Entry;

pub struct Config {
    path: String

}

impl Config {
    pub fn from(mut args: impl Iterator<Item=String>) -> Config {
        args.next();
        Config {
            path: args.next().unwrap_or(".".to_string())
        }
    }
}


/// The point where the program starts looking up for direcotries and files.
/// 
/// # Example
/// 
/// ```
/// let config = lnls::Config::from(env::args());
/// if let Err(error) = lnls::run(config) {
///    println!("Programm exited with the following error:\n{}", error);
///    process::exit(1);
/// }
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let entry = Entry::new(Path::new(&config.path))?;
    entry.print();
    Ok(())
}