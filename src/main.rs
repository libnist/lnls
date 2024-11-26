use std::process;
use std::env;
fn main() {
    let config = lnls::Config::from(env::args());
    if let Err(error) = lnls::run(config) {
        println!("Programm exited with the following error:\n{}", error);
        process::exit(1);
    }
}

