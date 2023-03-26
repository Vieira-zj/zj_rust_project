use std::env;
use std::error::Error;
use std::fs;
use std::process;

use crate::apps::minigrep::config as cfg;

pub fn run() {
    let config = cfg::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    });
    println!(
        "searching for [{}] in file [{}]:",
        &config.query, &config.file_path
    );

    if let Err(e) = read_file_and_search(config) {
        eprintln!("application error: {e}");
        process::exit(1);
    };
}

fn read_file_and_search(config: cfg::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[allow(dead_code)]
fn search_deprecated<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    #[test]
    fn query_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let got = super::search(query, contents);
        let want = vec!["safe, fast, productive."];
        assert_eq!(want, got);
    }

    #[test]
    fn query_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        let got = super::search_case_insensitive(query, contents);
        let want = vec!["Rust:", "Trust me."];
        assert_eq!(want, got);
    }
}
