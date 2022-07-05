use std::error::Error;
use std::fs::File;
// use std::io::ErrorKind;
// use std::io::{self, Read};
use std::fs;
use std::io;
use std::io::Read;

// fn main() {
// Recoverable Errors with `Result`

// The `Result` enum is defined as having two variants, `Ok` and `Err`.
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
// The `T` and `E` are generic type parameters.
// `T` represents the type of value that will be returned in a success case within
// the `Ok` variant, and `E` represents the type of the error that will be returned
// in a failure case within the `Err` variant.
// Because `Result` has these generic type parameters, we can use the `Result` type and
// the functions defined on it in many different situations where the successful value and
// error value we want to return may differ.

// let f = File::open("hello.txt");
// The generic parameter `T` has been filled in here with the type of the success value,
// `std::fs::File`, which is a file handle.
// The type of `E` used in the error value is `std::io::Error`.

// In the case where `File::open` succeeds, the value in the variable `f` will be an instance
// of `Ok` that contains a file handle. In the case where it fails, the value in `f` will be an
// instance of `Err` that contains more information about the kind of error that happened.
// let f = File::open("hello.txt");
// let f = match f {
//     Ok(file) => file,
//     Err(error) => panic!("Problem opening the file: {:?}", error)
// };

// Matching on Different Errors
// let f = File::open("hello.txt");
// let f = match f {
//     Ok(file) => file,
//     Err(error) => match error.kind() {
//         ErrorKind::NotFound => match File::create("hello.txt") {
//             Ok(fc) => fc,
//             Err(e) => panic!("Problem creating the file: {:?}", e)
//         }
//         other_error => {
//             panic!("Problem opening the file: {:?}", other_error)
//         }
//     }
// };
// Now the program panics on any error besides the missing file error.

// The `match` expression is very useful but also very much a primitive.
// Closures are used with many of the methods defined on `Result<T, E>`.
// These methods can be more concise than using `match` when handling
// `Result<T, E>` values in our code.
// let f = File::open("hello.txt").unwrap_or_else(|error| {
//     if error.kind() == ErrorKind::NotFound {
//         File::create("hello.txt").unwrap_or_else(|error| {
//             panic!("Problem creating the file: {:?}", error);
//         })
//     } else {
//         panic!("Problem opening the file: {:?}", error);
//     }
// });

// Shortcuts for Panic on Error: `unwrap` and `expect`
// The `Result<T, E>` type has many helper methods defined on it to do various,
// more specific tasks. The `unwrap` method is a shortcut method implemented just like
// the `match` expression. If the `Result` value is the `Ok` variant, `unwrap` will return
// the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will call the
// `panic!` macro for us.
// let f = File::open("hello.txt").unwrap();

// Similarly, the `expect` method lets us also choose the `panic!` error message. Using
// `expect` instead of `unwrap` and providing good error messages can convey your intent
// and make tracking down the source of a panic easier.
// let f = File::open("hello.txt").expect("Failed to open hello.txt");

// Propagating Errors
// When a function's implementation calls something that might fail, instead of handling
// the error within the function itself, we can return the error to the calling code so that
// it can decide what to do.
// This is known as *propagating* the error and gives more control to the calling code, where
// there might be more information or logic that dictates how the error should be handled.

// Where The `?` Operator Can Be Used
// The `?` operator is defined to perform an early return of a value out of the function.
// Therefore, the return type of the function has to be a `Result` so that it's compatible
// with this `return`.
// let f = File::open("hello.txt")?;  // Compile-time error.
// The `?` operator follows the `Result` value returned by `Fail::open`, but this `main`
// function has the return type of `()`, not `Result`.

// We are only allowed to use the `?` operator in a function that returns `Result`, `Option`,
// or another type that implements `FromResidual`.

// As with using `?` on `Result`, we can only use `?` on `Option` in a function that returns
// an `Option`. The behavior of the `?` operator when called on an `Option<T>` is:
// If the value is `None`, the `None` will be returned early from the function at that point.
// If the value is `Some`, the value inside the `Some` is the resulting value of the expression
// and function continues.

//     match last_char_of_first_line("Fuck the world!") {
//         Some(char) => println!("{}", char),
//         None => (),
//     }
//
//     let s = String::new();
//     match last_char_of_first_line(&s) {
//         Some(char) => println!("{}", char),
//         None => println!("Nothing!")
//     }
// }

// The `Box<dyn Error>` type is a *trait object*, which can be seen as "any kind of error."
// Using `?` on a `Result` value in a `main` function with the error type `Box<dyn Error>` is
// now allowed, because it allows any `Err` value to be returned early.
fn main() -> Result<(), Box<dyn Error>> {
    // `main` can also return a `Result<(), E>`.
    let f = File::open("hello.txt")?;

    Ok(())
    // When a `main` function returns a `Result<(), E>`, the executable will exit with a value of
    // `0` if `main` returns `Ok(())` and will exit with a nonzero value if `main` returns an `Err`
    // value.

    // The `main` function may return any types that implements the `std::process::Termination`
    // trait.
}

// Return type: `Result<String, io::Error>`. If the function succeeds without any problems, the code
// that calls this function will receive an `Ok` value that holds a `String`. If this function
// encounters any problems, the calling code will receive an `Err` value that holds an instance of
// `io::Error` that contains more information about what the problems were.
// We chose `io::Error` as the return type of this function because that happens to be the type of
// the error value returned from both of the operations we're calling in this function's body that
// might fail: the `File::open` function and the `read_to_string` method.
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e)
//         // Use the return keyword to return early out of the function entirely and pass the error
//         // value from `File::open`, now in the pattern variable `e`, back to the calling code as
//         // this function's error value.
//     };
//     let mut s = String::new();
//     // The `read_to_string` method also returns a `Result` because it might fail.
//     match f.read_to_string(&mut s) {  // Another `match` needed.
//         Ok(_) => Ok(s),
//         Err(e) => Err(e)  // No explicit `return` needed since this is the last expression.
//         // Return the error value in the same way that we returned the error value in the `match`
//         // that handled the return value of `File::open`.
//     }
// }
// It's up to the calling code to decide what to do with those values.

// A Shortcut for Propagating Errors: the `?` Operator
fn read_username_from_file() -> Result<String, io::Error> {
    // let mut f = File::open("hello.txt")?;  // Error occurs --> return early.
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;  // Same.
    // Ok(s)

    // Or even shorter:
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;  // Chain method calls.
    // Ok(s)

    // Or even shorter:
    fs::read_to_string("hello.txt")
    // Opens the file, creates a new `String`, reads the contents of the file, puts the contents
    // into that `String`, and returns it.
}
// The `?` placed after a `Result` value is defined to work in almost the same way as the `match`
// expressions we defined to handle the `Result` values. If the value of the `Result` is an `Ok`,
// the value inside the `Ok` will get returned from this expression, and the program will continue.
// If the value is an `Err`, the `Err` will be returned from the whole function as if we had used
// the `return` keyword so the error value gets propagated to the calling code.

// One difference: `error` values that have the `?` operator called on them go through the `from`
// function, defined in the `From` trait in the standard library, which is used to convert errors
// from one type into another.
// When the `?` operator calls the `from` function, the error type received is converted into the
// error type defined in the return type of the **current function**.
// This is useful when a function returns one error type to represent all the ways a function might
// fail, even if parts might fail for many different reasons.
// As long as there's an `impl From<OtherError> for ReturnedError` to define the conversion in the
// trait's `from` function, the `?` operator takes care of calling the `from` function automatically.

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
    // The `lines` method returns an iterator **over the lines** in the string.
    // Calls `next` on the iterator to get the first value from the iterator.
    // If `text` is the empty string, this call to `next` will return `None`,
    // in which case we use `?` to stop and return `None`.
    // If `text` is not the empty string, `next` will return a `Some` value containing a
    // string slice of the first line in `text`.

    // The `?` extracts the string slice, and we can call `chars` on that string slice to get an
    // **iterator of its characters**. We call `last` to return the last item in the iterator.
    // This is an `Option` because it's possible that the first line is the empty string.

    // Note: the `?` operator won't automatically convert a `Result` to an `Option` or vice versa;
    // in those cases, we can use methods like the `ok` method on `Result` or the `ok_or` method
    // on `Option` to do the conversion explicitly.
}
