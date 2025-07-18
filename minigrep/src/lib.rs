use std::{fs, result, vec};

use std::error::Error;
use std::env;

pub fn run(config: Config)->Result<(), Box<dyn Error>> {
     let contents = fs::read_to_string(config.filename)?;
     let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
      for line in results {
        println!("{}", line);
    }
     Ok(())
}

pub struct Config {
   pub query: String,
   pub  filename: String,
   pub  case_sensitive: bool,
}

impl Config {
  pub fn new(mut args:env::Args) -> Result<Config, &'static str> {
        /*if(args.len() < 3) {
            return Err("not enough arguments");
        }
        let query = args[1];
        let filename = args[2];*/
        args.next(); // Skip the first argument (the program name)
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
    
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
} 

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let mut results = Vec::new();
//     for line in contents.lines() {
//         if line.to_lowercase().contains(query.to_lowercase().as_str()) {
//         }
//     }
//    results
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
rust";
        assert_eq!(vec!["Rust:","rust"], search_case_insensitive(query, contents));
    }

   
}