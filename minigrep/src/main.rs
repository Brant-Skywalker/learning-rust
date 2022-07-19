#![allow(unused_doc_comments)]

use std::{env, process};

use minigrep::Config;

fn main() {
    /// ## Accepting Command Line Arguments
    ///
    /// The first task is to make `minigrep` accept its two command line arguments:
    /// the filename and a string to search for.
    ///
    /// ### Reading the Argument Values
    ///
    /// To enable `minigrep` to read the values of command line arguments we pass to it,
    /// we'll need the `std::env::args` function provided in Rust's standard library.
    /// This function returns an iterator of the command line arguments passed to `minigrep`.
    /// ```rust
    /// let args: Vec<String> = env::args().collect();  // Type annotation required here.
    /// println!("{:?}", args);
    /// ```
    ///
    /// Note that `std::env::args` will panic if any argument contains invalid Unicode.
    /// If our program needs to accept arguments containing invalid Unicode, use `std::env::args_os`
    /// instead. That function returns an iterator that produces `OsString` values instead of `String`
    /// values.
    ///
    /// ### Saving the Argument Values in Variables
    ///
    /// Now we need to save the values of the two arguments in variables so we can use the values
    /// throughout the rest of the program.
    /// ```rust
    /// let args: Vec<String> = env::args().collect();
    ///
    /// let query = &args[1];
    /// let filename = &args[2];
    ///
    /// println!("Searching for {}", query);
    /// println!("In file {}", filename);
    /// ```
    ///
    /// ## Reading a File
    ///
    /// Now we'll add functionality to read the file specified in the `filename` argument.
    /// ```rust
    /// let contents = fs::read_to_string(filename)
    ///     .expect("Something went wrong reading the file");
    ///
    /// println!("With text:\n{}", contents);
    /// ```
    ///
    /// ## Separation of Concerns of Binary Projects
    ///
    /// This process has the following steps:
    /// * Split our program into a *main.rs* and a *lib.rs* and move our program's logic to *lib.rs*.
    /// * As long as our command line parsing logic is small, it can remaining in *main.rs*.
    /// * When the command line parsing logic starts getting complicated, extract it from *main.rs*
    /// and move it to *lib.rs*.
    ///
    /// The responsibilities that remain in the `main` function after this process should be limited
    /// to the following:
    /// * Calling the command line parsing logic with the argument values
    /// * Setting up any other configuration
    /// * Calling a `run` function in *lib.rs*
    /// * Handling the error if `run` returns an error
    ///
    /// ### Extracting the Argument Parser
    ///
    /// ```rust
    /// fn parse_config(args: &[String]) -> (&str, &str) {
    ///     let query = &args[1];
    ///     let filename = &args[2];
    ///
    ///     (query, filename)
    /// }
    ///
    /// let args: Vec<String> = env::args().collect();
    /// let (query, filename) = parse_config(&args);
    /// ```
    ///
    /// ### Grouping Configuration Values
    ///
    /// ```rust
    /// fn parse_config(args: &[String]) -> Config {
    ///     let query = args[1].clone();
    ///     let filename = args[2].clone();
    ///
    ///     Config { query, filename }
    /// }
    ///
    /// let args: Vec<String> = env::args().collect();
    ///
    /// let config = parse_config(&args);
    /// ```
    ///
    /// ### Creating a Constructor for `Config`
    ///
    /// We can change `parse_config` from a plain function to a function named `new`
    /// that is associated with the `Config` struct.
    /// Making this change will make the code more idiomatic.
    /// ```rust
    /// let args: Vec<String> = env::args().collect();
    /// let config = Config::new(&args);
    /// ```
    ///
    /// ## Fixing the Error Handling
    ///
    /// ### Calling `Config::new` and Handling Errors
    ///
    /// ```rust
    /// let args: Vec<String> = env::args().collect();
    ///
    /// let config = Config::new(&args).unwrap_or_else(|err| {
    ///     eprintln!("Problem parsing arguments: {}", err);
    ///     process::exit(1);
    /// });
    ///
    /// println!("Searching for {}", config.query);
    /// println!("In file {}", config.filename);
    /// ```

    /// ### Extracting Logic from `main`
    ///
    /// ```rust
    /// fn run(config: Config) {
    ///     let contents = fs::read_to_string(config.filename)
    ///         .expect("Something went wrong reading the file");
    ///
    ///     println!("With text:\n{}", contents);
    /// }
    ///
    /// run(config);
    /// ```
    ///
    /// ### Returning Errors from the `run` Function
    ///
    /// The `run` function will return a `Result<T, E>` when something goes wrong.
    ///
    /// We use the *trait object* `Box<dyn Error>` for the error type. `Box<dyn Error>` means
    /// the function will return a type that implements the `Error` trait, but we don't have
    /// to specify what particular type the return value will be. This gives us flexibility to
    /// return error values that may be of different types in different error cases.
    /// The `dyn` keyword is short for "dynamic."
    ///
    /// ### Handling Errors Returned from `run` in `main`
    ///
    /// ```rust
    /// if let Err(e) = minigrep::run(config) {
    ///     eprintln!("Application error: {}", e);
    ///
    ///     process::exit(1);
    /// }
    /// ```
    ///
    /// ## Removing a `clone` Using an Iterator
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

