use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

/// We use the `var` function from the `env` module to check to see if any value
/// has been set for an environment variable named `IGNORE_CASE`.
///
/// The `env::var` returns a `Result` that will be successful `Ok` variant that contains the
/// value of the environment variable if the environment variable is set to any value. It will
/// return the `Err` variant if the environment variable is not set.
///
/// We're using the `is_ok` method on the `Result` to check whether the environment variable is
/// set, which means the program should do a case-insensitive search. If the `IGNORE_CASE` environment
/// variable isn't set to anything, `is_ok` will return false and the program will perform a
/// case-insensitive search.
///
/// ```rust
/// impl Config {
///     pub fn new(args: &[String]) -> Result<Config, &'static str> {
///         if args.len() < 3 {
///             return Err("not enough arguments");
///         }
///
///         let query = args[1].clone();
///         let filename = args[2].clone();
///
///         let ignore_case = env::var("IGNORE_CASE").is_ok();
///
///         Ok(Config {
///             query,
///             filename,
///             ignore_case,
///         })
///     }
/// }
/// ```
///
/// Now we update the definition of `Config::new` to pass ownership of the iterator
/// returned from `env::args` to `Config::new` directly.
///
/// The standard library documentation for the `env::args` function shows that the type of
/// the iterator it returns is `std::env::Args`, and that type implements the `Iterator` trait
/// and returns `String` values.
///
/// We've updated the signature of the `Config::new` function so the parameter `args` has a generic
/// type with the trait bounds `impl Iterator<Item = String>` instead of `&[String]`. This usage
/// of the `impl Trait` syntax means that `args` can be any type that implements the `Iterator`
/// type and returns `String` items.
impl Config {
    pub fn new(
        mut args: impl Iterator<Item=String>,
    ) -> Result<Config, &'static str> {
        // Using `Iterator` Trait Methods Instead of Indexing
        args.next();  // Ignore the first command line argument.

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

/// ### Iterating Through Lines with the `lines` Method
///
/// The `lines` method returns an iterator.
///
/// ```rust
/// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
///     let mut results = Vec::new();
///
///     for line in contents.lines() {
///         if line.contains(query) {
///             results.push(line);
///         }
///     }
///
///     results
/// }
/// ```
///
/// ## Making Code Clearer with Iterator Adaptors
///
/// The functional programming style prefers to minimize the amount of mutable
/// state to make code clearer. Removing the mutable state might enable a future
/// enhancement to make searching happen in parallel, because we wouldn't have
/// to manage concurrent access to the `results` vector.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }

    results
}

/// Now we add the search logic to the program using the test-driven development (TDD)
/// process with the following steps:
/// 1. Write a test that fails and run it to make sure it fails for the reason we expect.
/// 2. Write or modify just enough code to make the new test pass.
/// 3. Refactor the code we just added or changed and make sure the tests continue to pass.
/// 4. Repeat from step 1!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}