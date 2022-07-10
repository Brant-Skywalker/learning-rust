#![allow(unused_doc_comments)]

use crate::List::{Cons, Nil};

/// ```rust
/// enum List {
///     Cons(i32, List),
///     Nil,  // The non-recursive variant that signals the end of list.
/// }
/// ```
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    /// ## Using `Box<T>` to Point to Data on the Heap
    ///
    /// The most straight forward smart pointer is a *box*, whose type is written
    /// `Box<T>`.
    /// Boxes allow us to store data on the heap rather than the stack.
    /// What remains on the stack is the pointer to the heap data.
    ///
    /// Boxes don't have performance overhead, other than storing their data on the
    /// heap instead of on the stack.
    ///
    /// We use them most often in these situations:
    /// * When we have a type whose size can't be known at compile time and we want
    /// to use a value of that type **in a context that requires an exact size**.
    /// * When we have a large amount of data and we want to transfer ownership but
    /// **ensure the data won't be copied when we do so**.
    /// * When we want to own a value and we care only that it's a type that
    /// implements a particular trait rather than being of a specific type.
    ///
    /// The third case is known as a *trait object*.
    ///
    /// ### Using a `Box<T>` to Store Data on the Heap
    /// ```rust
    /// let b = Box::new(5);
    /// println!("b = {}", b);
    /// ```

    /// ### Enabling Recursive Types with Boxes
    ///
    /// A value of *recursive type* can have another value of the same type as
    /// part of itself. Recursive types pose an issue because at compile time Rust
    /// needs to know how much space a type takes up.
    /// However, the nesting of values of recursive types could theoretically
    /// continue infinitely, so Rust can't know how much space the value needs.
    /// Because boxes have a known size, we can enable recursive types by inserting
    /// a box in the recursive type definition.
    ///
    /// The *cons list* is a data type commonly found in functional programming languages.
    /// ```rust
    /// let list = Cons(1, Cons(2, Cons(3, Nil)));
    /// ```
    /// Rust cannot figure out how much space it needs to store a `List` value.

    /// ### Computing the Size of a Non-Recursive Type
    ///
    /// Because only one variant will be used, the most space an `enum` will need is the
    /// space it would take to store the largest of its variants.

    /// ### Using `Box<T>` to Get a Recursive Type with a Known Size
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);  // Cons(1, Cons(2, Cons(3, Nil)))
}
