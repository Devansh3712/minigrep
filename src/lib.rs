use std::error::Error;
use std::{fs, path};

use clap::Parser;
use regex::Regex;

#[derive(Debug, Parser)]
#[command(about = "Search for patterns in a file", long_about = None)]
pub struct Config {
    pub pattern: String,
    pub file_path: path::PathBuf,

    #[arg(short, long, action, help = "ignore case distinctions")]
    pub ignore_case: bool,

    #[arg(short, long, action, help = "pattern is a regular expression")]
    pub regex: bool,
}

// Box<dyn Error> returns a type that implements the Error
// trait. dyn keyword is short for Dynamic.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = match config {
        Config { ignore_case: true, .. } => search_case_insensitive(&config.pattern, &contents),
        Config { regex: true, .. } => {
            let regex = Regex::new(&config.pattern)?;
            search_regex(&regex, &contents)
        },
        _ => search(&config.pattern, &contents),
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// If a function returns a reference, lifetime parameters have
// to be added to its function signature
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_regex<'a>(pattern: &Regex, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if pattern.is_match(line) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
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
    fn case_sensitive() {
        let query = "tan";
        let contents = "hello secctan";

        assert_eq!(vec!["hello secctan"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "Tan";
        let contents = "hello secctan";

        assert_eq!(
            vec!["hello secctan"],
            search_case_insensitive(query, contents),
        );
    }
}
