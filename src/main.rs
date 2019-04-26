extern crate microgrep;

use std::env;
use std::process;

use microgrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("An error occurred while parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = microgrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}