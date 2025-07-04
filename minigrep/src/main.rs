use std::env;

use std::process;
use minigrep::{run, Config};


fn main() {
   // let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Query: {}", config.query);
    println!("Filename: {}", config.filename);
    if let Err(e) = run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}

