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
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
           return Err("not enough arguments");
        }
        let query = &args[1].clone();
        let filename = &args[2].clone();
        
        Ok(Self { query: query.to_string(), filename: filename.to_string() })
    }
   
}
