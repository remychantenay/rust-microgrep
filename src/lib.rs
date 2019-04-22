pub mod io;
pub mod search;

use std::error::Error;

pub struct Config {
    query: String,
    filename: String,
    case_insensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str>  {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_insensitive = args.len() > 3 && args[3] == "--i";

        Ok(Config {query, filename, case_insensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {

    let contents = io::file_to_string(&config.filename)?;
    
    let mut search_results: Vec<&str>;
    if config.case_insensitive {
        search_results = search::search_case_insensitive(&config.query, &contents);
    } else {
        search_results = search::search(&config.query, &contents);
    }

    for line_search_results in search_results {
            println!("{}", line_search_results);
    }

    Ok(())
}