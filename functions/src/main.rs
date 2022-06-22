fn main() {
    println!("Hello, world!");

    // another_function();

    another_function(5);

    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

/// Rust code uses *snake case* as the conventional style for function and variable names,
/// in which all letters are lowercase and underscores separate words.
/// Rust **doesn't care** where we define our functions.
// fn another_function() {
//     println!("Another function.");
// }

/// ### Parameters
///
/// In function signatures, we *must* declare the type of each parameter.
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

/// Separate the parameter declarations with commas.
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

/// ### Statements and Expressions
/// *Statements* are instructions that perform some action and do not return a value.
/// *Expressions* evaluate to a resulting value.
fn statement_and_expressions() {
    // let x = (let y = 6); // The `let y = 6` statement does not return a value!

    let y = {
        let x = 3;
        x + 1  // Expressions do not include ending semicolons.
    };
}

/// ### Functions with Return Values
///
/// We must declare the type of a return value after an arrow (`->`).
/// In Rust, the return value of the function is synonymous with the value of the final
/// expression in the block of the body of a function.
///
/// We can return early from a function by using the `return` keyword and
/// specifying a value.

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// fn plus_one(x: i32) -> i32 {
//     x + 1;  // Returns the unit type `()`.
// }
