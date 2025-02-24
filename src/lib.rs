// lib.rs
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for '{}' in file '{}'", config.query, config.filename);

    let contents = fs::read_to_string(config.filename)?;

    if config.mode == "-i" {
        let results = search_case_insensitive(&config.query, &contents);
        for line in results {
            println!("{}", line);
        }
    } else if config.mode == "-n" {
        let count = search_count(&config.query, &contents);
        println!("Found {} occurrences of '{}'", count, config.query);
    } else {
        let results = search(&config.query, &contents);
        for line in results {
            println!("{}", line);
        }
    }

    Ok(())
}

pub struct Config {
    pub mode: String,
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let mode;
        let query;
        let filename;

        if args.len() == 3 {
            mode = String::from("-d"); // Default mode
            query = args[1].clone();
            filename = args[2].clone();
        } else if args.len() == 4 {
            mode = args[1].clone(); // Mode is explicitly given
            query = args[2].clone();
            filename = args[3].clone();
        } else {
            return Err("not enough arguments");
        }

        // Ensure that the mode is either "-i", "-d", or "-n"
        if mode != "-i" && mode != "-d" && mode != "-n" {
            return Err("invalid mode argument");
        }

        Ok(Config { query, filename, mode })
    }
}

pub fn search_count(query: &str, contents: &str) -> i32 {
    let mut c = 0;

    for line in contents.lines() {
        if line.contains(query) {
            c += 1;
        }
    }

    c
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "duct";
        let contents: &str = "\
Rust:
safe, secure, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, secure, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "RuSt";
        let contents: &str = "\
Rust:
safe, secure, productive.
Pick three.
Trust me";

        assert_eq!(
            vec!["Rust:", "Trust me"],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn count_occurrences() {
        let query: &str = "duct";
        let contents: &str = "\
Rust:
safe, secure, productive.
Pick three.
Duct tape.";

        assert_eq!(1, search_count(query, contents));
    }
}