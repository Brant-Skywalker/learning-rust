#![allow(unused_doc_comments)]

use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preferences: Option<ShirtColor>) -> ShirtColor {
        user_preferences.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    /// # Closures: Anonymous Functions that Can Capture Their Environment
    ///
    /// Rust's closures are anonymous functions we can save in a variable or pass
    /// as arguments to other functions. We can create the closure in one place and
    /// then call the closure to evaluate it in a different context.
    /// Unlike functions, closures can capture values from the scope in which they're
    /// defined.
    ///
    /// ## Capturing the Environment with Closures
    ///
    /// ```rust
    /// let store = Inventory {
    ///     shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    /// };
    ///
    /// let user_pref1 = Some(ShirtColor::Red);
    /// let giveaway1 = store.giveaway(user_pref1);
    /// println!("The user with preferences {:?} gets {:?}", user_pref1, giveaway1);
    ///
    /// let user_pref2 = None;
    /// let giveaway2 = store.giveaway(user_pref2);
    /// println!("The user with preferences {:?} gets {:?}", user_pref2, giveaway2);
    /// ```
    /// The `giveaway` method takes the user preference `Option<ShirtColor>` and calls
    /// `unwrap_or_else` on it. The `unwrap_or_else` method takes one argument: a closure
    /// without any arguments that returns a value `T` (the same type stored in the `Some`
    /// variant of the `Option<T>`, in this case, a `ShirtColor`). If the `Option<T>` is the
    /// `Some` variant, `unwrap_or_else` returns the value from within the `Some`. If the
    /// `Option<T>` is the `None` variant, `unwrap_or_else` calls the closure and returns
    /// the value returned by the closure.
    ///
    /// This is interesting because we've passed a closure that calls `self.most_stocked()`
    /// on the current `Inventory` instance. The standard library didn't need to know anything
    /// about the `Inventory` or `ShirtColor` types we defined, or the logic we want to use in
    /// this scenario. The closure captured an immutable reference to the `self` `Inventory`
    /// instance and passed it with the code we specified to the `unwrap_or_else` method.
    /// Functions are not able to capture their environment in this way.
    ///
    /// ## Closure Type Inference and Annotation
    ///
    /// There are more differences between functions and closures. Closures don't usually require
    /// us to annotate the types of the parameters or the return value like `fn` functions do.
    /// Type annotations are required on functions because they're part of an explicit interface
    /// exposed to our users. Defining this interface rigidly is important for ensuring that
    /// everyone agrees on what types of values a function uses and returns. But closures aren't
    /// used in an exposed interface like this: they're stored in variables and used without
    /// naming them and exposing them to users of our library.
    ///
    /// As with variables, we can add type annotations if we want to increase explicitness and
    /// clarity at the cost of being more verbose than is strictly necessary.
    ///
    /// ```rust
    /// let expensive_closure = |num: u32| -> u32{
    ///     println!("calculating slowly...");
    ///     thread::sleep(Duration::from_secs(2));
    ///     num
    /// };
    /// expensive_closure(1);
    /// ```
    ///
    /// With type annotations added, the syntax of closures looks more similar to the syntax of
    /// functions. The following is a vertical comparison of the syntax for the definition of a
    /// function that adds 1 to its parameter and a closure that has the same behavior.
    /// ```rust
    /// fn add_one_v1(x: u32) -> u32 { x + 1 }
    /// let add_one_v2 = |x: u32| -> u32{ x + 1 };
    /// let add_one_v3 = |x| { x + 1 };
    /// let add_one_v4 = |x| x + 1;
    /// ```
    /// These are all valid definitions that will produce the same behavior when they're called.
    /// Calling the closures is required for `add_one_v3` and `add_one_v4` to be able to compile
    /// because the types will be inferred from their usage.
    ///
    /// Closure definitions will have **one** concrete type inferred for each of their parameters
    /// and for their return value.
    /// ```rust
    /// let example_closure = |x| x;
    /// let s = example_closure(String::from("hello"));
    /// let n = example_closure(5);  // Compiler error!
    /// ```
    ///
    /// ## Capturing References or Moving Ownership
    ///
    /// Closures can capture values from their environment in three ways, which directly map to
    /// the three ways a function can take a parameter: borrowing immutably, borrowing mutably,
    /// and taking ownership. The closure will decide which of these to use based on what the body
    /// of the function does with the captured values.
    /// ```rust
    /// let list = vec![1, 2, 3];
    /// println!("Before defining closure: {:?}", list);
    ///
    /// let only_borrows = || println!("From closure: {:?}", list);
    ///
    /// println!("Before calling closure: {:?}", list);
    /// only_borrows();
    /// println!("After calling closure: {:?}", list);
    /// ```
    /// The closure only needs an immutable borrow to print the value.
    ///
    /// Now we change the closure definition to need a mutable borrow because the closure body
    /// adds an element to the `list` vector:
    /// ```rust
    /// let mut list = vec![1, 2, 3];
    /// println!("Before defining closure: {:?}", list);
    ///
    /// let mut borrows_mutably = || list.push(7);
    ///
    /// borrows_mutably();
    ///
    /// println!("After calling closure: {:?}", list);
    /// ```
    ///
    /// When `borrows_mutably` is defined, it captures a mutable reference to `list`. After the
    /// closure is called, because we don't use the closure again after that point, the mutable
    /// borrow ends. Between the closure definition and the closure call, an immutable borrow to
    /// print isn't allowed because no other borrows are allowed when there's a mutable borrow.
    ///
    /// If we want to force the closure to take ownership of the values it uses in the environment
    /// even though the body of the closure doesn't strictly need ownership, we can use the `move`
    /// keyword before the parameter list. This technique is mostly useful when passing a closure
    /// to a new thread to move the data so it's owned by the new thread.
    ///
    /// ## Moving Captured Values Out of the Closure and the `Fn` Traits
    ///
    /// Once a closure has captured a reference or moved a value into the closure, the code in
    /// the body of the function also affects what happens to the references or values as a result
    /// of calling the function. A closure body can move a captured value out of the closure, can
    /// mutate the captured value, can neither move nor mutate the captured value, or can capture
    /// nothing from the environment.
    ///
    /// The way a closure captures and handles values from the environment affects which traits the
    /// closure implements. The traits are how functions and structs can specify what kinds of
    /// closures they can use.
    ///
    /// Closures will automatically implement one, two, or all three of these `Fn` traits,
    /// in an additive fashion:
    /// 1. `FnOnce` applies to closures that can be called at least once. All closures implement
    /// this trait, because all closures can be called. If a closure moves captured values out of
    /// its body, then that closure only implements `FnOnce` and not any of the other `Fn` traits,
    /// because it can only be called once.
    /// 2. `FnMut` applies to closures that don't move captured values out of their body, but that
    /// might mutate the captured values. These closures can be called more than once.
    /// 3. `Fn` applies to closures that don't move captured values out of their body and that
    /// don't mutate captured values. These closures can be called more than once without mutating
    /// their environment, which is important in cases such as calling a closure multiple times
    /// concurrently. Closures that don't capture anything from their environment implement `Fn`.
    ///
    /// ```rust
    /// impl<T> Option<T> {
    ///     pub fn unwrap_or_else<F>(self, f: F) -> T
    ///         where
    ///             F: FnOnce() -> T
    ///     {
    ///         match self {
    ///             Some(x) => x,
    ///             None => f(),
    ///         }
    ///     }
    /// }
    /// ```
    /// The trait bound specified on the generic type `F` is `FnOnce() -> T`, which means `F`
    /// must be able to be called at least once, take no arguments, and return a `T`.
    /// Using `FnOnce` in the trait bound expresses the constraint that `unwrap_or_else` is
    /// only going to call `f` at most one time. Because all closures implements `FnOnce`,
    /// `unwrap_or_else` accepts the most different kinds of closures and is as flexible as it
    /// can be.
    ///
    /// Note: Functions can implement all three of the `Fn` traits too. If what we want to do
    /// doesn't require capturing a value from the environment, we can use the name of a function
    /// rather than a closure where we need something that implements one of the `Fn` traits.
    /// For example, on an `Option<Vec<T>>` value, we could call `unwrap_or_else(Vec::new)` to get
    /// a new, empty vector if the value is `None`.
    ///
    /// Now let's look at the standard library method `sort_by_key` defined on slices, to see
    /// how that differs. It takes a closure that implements `FnMut`. The closure gets one argument,
    /// a reference to the current item in the slice being considered, and returns a value of
    /// type `K` that cna be ordered.
    ///
    /// ```rust
    /// let mut list = [
    ///     Rectangle {
    ///         width: 10,
    ///         height: 1,
    ///     },
    ///     Rectangle {
    ///         width: 3,
    ///         height: 5,
    ///     },
    ///     Rectangle {
    ///         width: 7,
    ///         height: 12,
    ///     }
    /// ];
    ///
    /// list.sort_by_key(|r| r.width);
    /// println!("{:#?}", list);
    /// ```
    /// The reason `sort_by_key` is defined to take an `FnMut` closure is that it calls the closure
    /// multiple times: once for each item in the slice. The closure `|r| r.width` doesn't capture,
    /// mutate, or move out anything from its environment, so it meets the trait bound requirements.
    /// ```rust
    /// let mut list = [
    ///     Rectangle {
    ///         width: 10,
    ///         height: 1,
    ///     },
    ///     Rectangle {
    ///         width: 3,
    ///         height: 5,
    ///     },
    ///     Rectangle {
    ///         width: 7,
    ///         height: 12,
    ///     },
    /// ];
    ///
    /// let mut sort_operations = vec![];
    /// let value = String::from("by key called");
    ///
    /// list.sort_by_key(|r| {
    ///     sort_operations.push(value);
    ///     r.width
    /// });
    /// println!("{:#?}", list);
    /// ```
    /// The closure here only implements `FnOnce` because it moves a value out of the environment.
    /// The compiler won't let us use this closure with `sort_by_key`.
    ///
    /// The following code works with `sort_by_key` because it is only capturing a mutable reference
    /// to the `num_sort_operations` counter and can therefore be called more than once:
    ///
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {} operations", list, num_sort_operations);
    /// The `Fn` traits are important when defining or using functions or types that make use of
    /// closures.
    ()
}

