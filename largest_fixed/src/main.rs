// Operator is defined as a default method on the standard library trait
// `std::cmp::PartialOrd`, we need to specify `PartialOrd` in the trait bounds
// for `T` so the `largest` function can work on slices of any type that we can
// compare. We don't need to bring `PartialOrd` into scope because it's in the
// prelude.
// To call this code with only those types that implement the `Copy` trait, we can
// add `Copy` to the trait bounds of `T`!
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];
//
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }

// If we don't want to restrict the `largest` function to the types that implement
// the `Copy` trait, we could specify that `T` has the trait bound `Clone` instead
// of `Copy`.
fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list {
        if *item > largest {
            largest = item.clone();
        }
    }

    largest
}

// We could also implement `largest` by having the function return a reference to
// a `T` value in the slice. If we change the return type to `&T` instead of `T`,
// thereby changing the body of the function to return a reference, we wouldn't
// need the `Clone` or `Copy` trait bounds and we could avoid heap allocations.
// fn largest<T: PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // Because the standard library has this blanket implementation, we can call
    // the `to_string` method defined by the `ToString` trait on any type that
    // implements the `Display` trait.
    let s = 3.to_string();

}

// Using Trait Bounds to Conditionally Implement Methods
// We can implement methods conditionally for types that implement the
// specified traits.
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// In this `impl` block, `Pair<T>` only implements the `cmp_display` method if
// its inner type `T` implements the `PartialOrd` trait that enables comparison
// *and* the `Display` trait that enables printing.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can also conditionally implement a trait for any type that implements another trait.
// Implementations of a trait on any type that satisfies the trait bounds are called
// *blanket implementations* and are extensively used in the Rust standard library.
// impl<T: Display> ToString for T {
//     // --snip--
// }

