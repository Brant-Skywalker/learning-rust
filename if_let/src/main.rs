use std::vec::from_elem;

fn main() {
    // Concise Control Flow with `if let`
    // The `if let` syntax lets us combine `if` and `let` into a less verbose way to
    // handle values that match one pattern while ignoring the rest.
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => ()  // Annoying boilerplate code to add.
    }

    // Instead, we could write this in a shorter way using `if let`.
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // The code in the `if let` block isn't run if the value doesn't match the pattern.

    // Using `if let` means less typing, less indentation, and less boilerplate code.
    // However, we lose the exhaustive checking that `match` enforces.
    // We can think of `if let` as syntax sugar for a `match` that runs code when the
    // value matches on pattern and then ignores all other values.

    // We can include an `else` with an `if let`. The block of code that goes with the
    // `else` is the same as the block of code that would go with the `_` case in the
    // `match` expression that is equivalent to the `if let` and `else`.
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    // If we wanted to count all non-quarter coins we see while also announcing
    // the state of quarters, we could do that with a match expression like this:
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1
    }

    // Or we could use an `if let` and `else` expression like this:
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
