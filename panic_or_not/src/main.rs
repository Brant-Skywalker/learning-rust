fn main() {
    // Returning `Result` is a good default choice when we're defining a function
    // that might fail.

    // In situations such as examples, prototype code, and tests, it's more appropriate
    // to write code that panics instead of returning a `Result`.

    // The `unwrap` and `expect` methods are very handy when prototyping. They leave
    // clear markers in our code for when we're ready to make our program more robust.

    // If a method call fails in a test, we'd want the whole test to fail, even if that
    // method isn't the functionality under test. Because `panic!` is how a test is marked
    // as a failure, calling `unwrap` or `expect` is exactly what should happen.

    // It would also be appropriate to call `unwrap` when we have some other logic that ensures
    // the `Result` will have an `Ok` value, but the logic isn't something the compiler understands.
    // If we can ensure by manually inspecting the code that we'll never have an `Err` variant, it's
    // perfectly acceptable to call `unwrap`.
    use std::net::IpAddr;
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    // We can see that `127.0.0.1` is a valid IP address, so it's acceptable to use `unwrap` here.
    // If the IP address string came from a user rather than being hardcoded into the program and
    // therefore *did* have a possibility of failure, we'd definitely want to handle the `Result`
    // in a more robust way instead.

    // Creating Custom Types for Validation
    // We can make a new type and put the validations in a function to create an instance of the
    // type rather than repeating the validations everywhere. That way, it's safe for functions
    // to use the new type in their signatures and confidently use the values they receive.
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            // Alert the programmer who is writing the calling code that they have a bug they
            // need to fix.
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    // This kind of method is sometimes called a *getter*.
    // This public method is necessary because the `value` field of the `Guess` struct is private.
    pub fn value(&self) -> i32 {
        self.value
    }
}