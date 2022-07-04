fn main() {
    // Unrecoverable Errors with `panic!`
    // When the `panic!` macro executes, our program will print a failure message,
    // unwind and clean up the stack, and then quit.

    // By default, when a panic occurs, the program starts *unwinding*, which means Rust
    // walks back up the stack and cleans up the data from each function it encounters.

    // Rust also allows us to choose the alternative of immediately *aborting*, which ends
    // the program without cleaning up.
    // If we need to make the resulting binary as small as possible, we can switch from 
    // unwinding to aborting upon a panic by adding `panic = 'abort'` to the appropriate 
    // `[profile]` sections in our `Cargo.toml` file.

    // panic!("crash and burn");

    // Using a `panic!` Backtrace
    let v = vec![1, 2, 3];
    v[99];
    // A *backtrace* is a list of all the functions that have been called to get to this
    // point. The key to reading the backtrace is to start from the top and read until we
    // see files we wrote.

    // Let's try getting a backtrace by setting the `RUST_BACKTRACE` environment variable
    // to any value except 0.
    // `RUST_BACKTRACE=1 cargo run`
}
