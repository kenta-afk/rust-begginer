use std::env;
use std::process;

use mini::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", &args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    // println!("Searching for {}", query);
    // println!("In file {}", filename);
    if let Err( err ) = mini::run(config){
        println!("Application error: {}", err);
        process::exit(1);
    };
}


