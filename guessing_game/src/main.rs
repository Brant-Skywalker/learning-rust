use std::io;  // Bring the `io` input/output library into scope.

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();  // Bounded to a new, empty instance of a `String`.

    io::stdin()
        .read_line(&mut guess) // Pass a mutable reference to `read_line`.
        .expect("Failed to read line");

    // `read_line` here returns a `io::Result` value. An instance of `io::Result` has an `expect`
    // method that you can call. If this instance of `io::Result` is an `Err` value, `expect` will
    // cause the program to crash and display the message we passed as an argument to `expect`.
    // If the `read_line` method returns an `Err`, it would likely be the result of an error coming
    // from the underlying operating system. If this instance of `io::Result` is an `Ok` value,
    // `expect` will take the return value that `Ok` is holding and return just that value.

    println!("You guessed: {}", guess); // The `{}` is a placeholder.
}
