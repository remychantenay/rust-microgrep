use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn file_to_string(filename: &str) -> Result<String, Box<Error>> {
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    
    Ok(contents)
}