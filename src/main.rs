use std::path::Path;
use std::process;
use std::env;
fn main() {
    let paths: Vec<String> = env::args().collect();
    let path = Path::new(&paths[1]);
    if let Err(error) = lnls::run(path) {
        println!("Programm exited with the following error:\n{}", error);
        process::exit(1);
    }
}

