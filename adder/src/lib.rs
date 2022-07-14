#![allow(unused_doc_comments)]

/// ## How to Write Tests
///
/// Tests are Rust functions that verify that the non-test code is functioning
/// in the expected manner. The bodies of test functions typically perform these
/// three actions:
/// 1. Set up any needed data or state.
/// 2. Run the code we want to test.
/// 3. Assert the results are what we expect.
///
/// ### The Anatomy of a Test Function
///
/// At its simplest, a test in Rust is a function that's annotated with the
/// `test` attribute. Attributes are metadata about pieces of Rust code; one
/// example is the `derive` attribute we used with structs.
///
/// To change a function into a test function, add `#[test]` on the line before
/// `fn`. When we run our tests with the `cargo test` command, Rust builds a
/// test runner binary that runs the annotated functions and reports on whether
/// each test function passes or fails.
#[cfg(test)]
mod tests {
    /// This attribute indicates this is a test function.
    // #[test]
    // fn it_works() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);
    // }
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    use std::f32::consts::E;
    /// Each test is run in a new thread, and when the main thread sees that a test
    /// thread has died, the test is marked as failed.
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    /// The `tests` module is a regular module that follows the usual visibility rules.
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    /// The `assert_ne!` macro is most useful for cases when we're not sure what a value *will*
    /// be, but we know what the value definitely *shouldn't* be.

    /// ## Adding Custom Failure Messages
    ///
    /// Any arguments specified after the required arguments of `assert!`, `assert_eq!`,
    /// and `assert_ne!` macros will be passed along to the `format!` macro, so we can
    /// pass a format string that contains `{}` placeholders and values go in those placeholders.
    /// ```rust
    /// #[test]
    /// fn greeting_contains_name() {
    ///     let result = greeting("Carol");
    ///     assert!(
    ///         result.contains("Carol"),
    ///         "Greeting did not contain a name, value was `{}`",
    ///         result
    ///     );
    /// }
    /// ```
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    /// ## Using `Result<T, E>` in Tests
    ///
    /// We can also write tests that use `Result<T, E>`!
    /// In the body of the function, rather than calling the `assert_eq!` macro,
    /// we return `Ok(())` when the test passes and an `Err` with a `String` inside
    /// when the test fails.
    ///
    /// Writing tests so they return a `Result<T, E>` enables us to use the question
    /// mark operator in the body of tests, which can be a convenient way to write tests
    /// that should fail if any operation within them returns an `Err` variant.
    ///
    /// We can't use the `#[should_panic]` annotation on tests that use `Result<T, E>`.
    /// To assert that an operation returns an `Err` variant, *don't* use the question
    /// mark operator on the `Result<T, E>` value. Instead, use `assert!(value.is_err())`.
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    // format!("Hello {}!", name)
    String::from("Hello!")
}

/// ## Checking for Panics with `should_panic`
///
/// It's important to check that our code handles error conditions as we expected.
/// We do this by adding the attribute `should_panic` to our test function.
///
/// The test passes if the code inside the function panics; the test fails if the
/// code inside the function doesn't panic.

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}