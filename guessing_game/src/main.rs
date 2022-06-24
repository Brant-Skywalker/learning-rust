use std::io;
// Bring the `io` input/output library into scope.
use rand::Rng;
// The `Rng` trait defines methods that random number generators implement.
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // This value also inferred to be a `u32` according to the comparison below.
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();  // Bounded to a new, empty instance of a `String`.

        io::stdin()
            .read_line(&mut guess) // Pass a mutable reference to `read_line`.
            .expect("Failed to read line");

        // // The `trim` method on a `String` instance will eliminate any whitespace at the beginning and end.
        // // The `parse` method on a strings parses a string into some kind of number.
        // // The `parse` method returns a `Result` type, much as the `read_line` does.
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");  // Shadowing!

        // *Handling Invalid Input*
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,  // `_` is a catchall value.
        };

        // `read_line` here returns a `io::Result` value. An instance of `io::Result` has an `expect`
        // method that you can call. If this instance of `io::Result` is an `Err` value, `expect` will
        // cause the program to crash and display the message we passed as an argument to `expect`.
        // If the `read_line` method returns an `Err`, it would likely be the result of an error coming
        // from the underlying operating system. If this instance of `io::Result` is an `Ok` value,
        // `expect` will take the return value that `Ok` is holding and return just that value.

        println!("You guessed: {}", guess); // The `{}` is a placeholder.

        // The `cmp` method compares two values and can be called on anything that can be compared.
        // It takes a reference to whatever you want to compare with.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
