fn main() {
    /// ## Control Flow
    ///
    /// **`if` Expressions**
    /// An `if` expression start with the keyword `if`, followed by a condition.
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    /// The condition in this code *must* be a `bool`.
    // let number = 3;
    // if number {  // Error!
    //     println!("number was three");
    // }

    /// Rust will not automatically try to convert non-Boolean types to a Boolean.
    /// Use this:
    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }

    /// Handling Multiple Conditions with `else if`
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    /// A powerful Rust branching construct: `match`.

    /// Using `if` in a `let` Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // let number = if condition { 5 } else { "six" };  // Types mismatched!

    /// Repeating code with `loop`.
    // loop {
    //     println!("again!");
    // }

    /// We can optionally specify a *loop label* on a loop that we can then use
    /// with `break` or `continue` to specify that those keywords apply to the
    /// labeled loop instead of the innermost loop.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 0 {
                break;
            }
            if count == 2 {
                break 'counting_up;  // `break` here applies to the outer loop.
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    /// *Returning Values from Loops*
    /// Add the value we want returned after the `break` expression we use
    /// to stop the loop.
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // Pass the result of the operation out of the loop.
        }
    };
    println!("The result is {}", result);

    /// *Conditional Loops with `while`*
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    /// *Looping Through a Collection with `for`*
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        /// Error prone. May panic.
        println!("the value is: {}", a[index]);
        index += 1;
    }

    /// A more concise alternative:
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    /// Use a `Range`.
    for number in (1..4).rev() {  // Use method `rev` to reverse the range.
        println!("{}!", number);
    }
    println!("LIFTOFF");
}

