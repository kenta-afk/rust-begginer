use std::io::prelude::*;
use std::error::Error;
use std::fs::File;

pub struct Config {
    pub query: String,
    pub filename: String,
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>>{
    
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    

    println!("With text:\n{}", contents);
    Ok(())
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        
        args.next();

        let query = match args.next() {
            Some(query) => query,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(filename) => filename,
            None => return Err("Didn't get a file name"),
        };
        
        Ok(Self { query: query.to_string(), filename: filename.to_string() })
    }
   
}
