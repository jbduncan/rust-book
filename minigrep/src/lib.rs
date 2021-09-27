//! # Minigrep
//!
//! `minigrep` is a very small, very minimalist grep-like tool.
//!
//! See `run` for an example of how to run this library as a command-line tool.

use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // Discard the program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive =
            env::var("CASE_INSENSITIVE").is_err() && does_not_contain(args.next(), "--case-insensitive");

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

fn does_not_contain(arg: Option<String>, value: &str) -> bool {
    arg.is_none() || arg.unwrap() != value
}

/// Runs `minigrep` as part of your Rust program.
///
/// # Example
///
/// ```
/// fn run() -> i32 {
///     let config = match minigrep::Config::new(std::env::args()) {
///         Ok(config) => config,
///         Err(err) => {
///             eprintln!("Problem parsing arguments: {}", err);
///             return 1;
///         }
///     };
///
///     if let Err(e) = minigrep::run(config) {
///         eprintln!("Application error: {}", e);
///         return 1;
///     }
///
///     return 0;
/// }
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_ascii_lowercase();
    return contents
        .lines()
        .filter(|line| line.to_ascii_lowercase().contains(&query))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    mod given_file_contents_and_a_query_that_matches_it {
        use super::*;

        const QUERY: &'static str = "duct";
        const CONTENTS: &'static str = indoc! { "
            Rust:
            safe, fast, productive.
            Pick three.
            Duct tape."
        };

        mod when_running_a_search {
            use super::*;

            #[test]
            fn then_it_returns_matching_line() {
                let result = search(QUERY, CONTENTS);
                assert_eq!(vec!["safe, fast, productive."], result);
            }
        }

        mod when_running_a_case_insensitive_search {
            use super::*;

            #[test]
            fn then_it_returns_matching_lines() {
                let result = search_case_insensitive(QUERY, CONTENTS);
                assert_eq!(vec!["safe, fast, productive.", "Duct tape."], result);
            }
        }
    }

    mod given_empty_file_contents_and_any_query {
        use super::*;

        const QUERY: &'static str = "any old query";
        const CONTENTS: &'static str = "";

        mod when_running_a_search {
            use super::*;

            #[test]
            fn then_it_returns_nothing() {
                let result = search(QUERY, CONTENTS);
                assert!(result.is_empty())
            }
        }

        mod when_running_a_case_insensitive_search {
            use super::*;

            #[test]
            fn then_it_returns_nothing() {
                let result = search_case_insensitive(QUERY, CONTENTS);
                assert!(result.is_empty())
            }
        }
    }

    mod given_any_file_contents_and_an_empty_query {
        use super::*;

        const QUERY: &'static str = "";
        const CONTENTS: &'static str = "any old file contents";

        mod when_running_a_search {
            use super::*;

            #[test]
            fn then_it_returns_all_lines() {
                let result = search(QUERY, CONTENTS);
                assert_eq!(result.join("\n"), CONTENTS)
            }
        }

        mod when_running_a_case_insensitive_search {
            use super::*;

            #[test]
            fn then_it_returns_all_lines() {
                let result = search_case_insensitive(QUERY, CONTENTS);
                assert_eq!(result.join("\n"), CONTENTS)
            }
        }
    }
}
