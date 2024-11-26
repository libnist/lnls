use std::path::Path;
use std::process;
fn main() {
    if let Err(error) = lnls::run(Path::new(".")) {
        println!("Programm exited with the following error:\n{}", error);
        process::exit(1);
    }
}

