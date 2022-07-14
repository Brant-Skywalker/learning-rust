#![allow(unused_doc_comments)]

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use rand::Rng;
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        // Or:
        super::hosting::add_to_waitlist();
    }
}

fn main() {
    /// # Bringing Paths into Scope with the `use` Keyword
    ///
    /// We can create a shortcut to a path with the `use` keyword once, and then
    /// use the shorter name everywhere else in the scope.
    ///
    /// Adding `use` and a path in the scope is similar to creating a symbolic link
    /// in the filesystem.
    ///
    /// Note that `use` only creates the shortcut for the particular scope in which
    /// the `use` occurs.
    ///
    /// ## Creating Idiomatic `use` Paths
    ///
    /// Bringing the function's parent module into the scope with `use` means we have
    /// to specify the parent module when calling the function. Specifying the parent
    /// module when calling the function. Specifying the parent module when calling
    /// the function makes it clear that the function isn't locally defined while still
    /// minimizing repetition of the full path.
    ///
    /// On the other hand, when bringing in structs, enums, and other items with `use`,
    /// it's idiomatic to specify the full path.
    /// ```rust
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, 2);
    /// ```
    ///
    /// The exception to this idiom is if we're bringing two items with the same
    /// name into scope with `use` statements, because Rust doesn't allow that.
    ///
    /// ## Providing New Names with the `as` Keyword
    ///
    /// After the path, we can specify `as` and a new local name, or alias, for the type.
    /// ```rust
    /// use std::fmt::Result;
    /// use std::io::Result as IoResult;
    ///
    /// fn function1() -> Result {
    ///     // --snip--
    /// }
    ///
    /// fn function2() -> IoResult<()> {
    ///     // --snip--
    /// }
    /// ```
    ///
    /// ## Re-exporting Names with `pub use`
    ///
    /// When we bring a name into scope with the `use` keyword, the name available in the new
    /// scope is private. To enable the code that calls our code to refer to that name as if
    /// it had been defined in that code's scope, we can combine `pub` and `use`. This technique
    /// is called *re-exporting* because we're bringing an item into scope but also making that
    /// item available for others to bring into their scope.
    /// ```rust
    /// pub use crate::front_of_house::hosting;
    /// ```
    ///
    /// Re-exporting is useful when the internal structure of our code is different from how
    /// programmers calling our code would think about the domain.
    ///
    /// With `pub use`, we can write our code with one structure but expose a different structure.
    ///
    /// ## Using External Packages
    /// ```rust
    /// use rand::Rng;
    ///
    /// let secret_number = rand::thread_rng().gen_range(1..=100);
    /// ```
    ///
    /// Members of the Rust community have made many packages available at `crate.io`, and pulling
    /// any of them into our package involves these steps: listing the in our package's `Cargo.toml`
    /// file and using `use` to bring items from their crates into scope.
    ///
    /// Note that the standard library (`std`) is also a crate that's external to our package.
    /// Because the standard library is shipped with the Rust language, we don't need to change
    /// `Cargo.toml` to include `std`. But we do need to refer to it with `use` to bring items
    /// from there into our package's scope.
    ///
    /// ## Using Nested Paths to Clean Up Large `use` Lists
    ///
    /// We can use nested paths to bring the same items into scope in one line.
    /// ```rust
    /// use std::{cmp::Ordering, io};
    /// ```
    ///
    /// We can use a nested path at any level in a path, which is useful when combining two `use`
    /// statements that share a subpath.
    /// ```rust
    /// use std::io;
    /// use std::io::Write;
    /// ```
    /// is equivalent to:
    /// ```rust
    /// use std::io::{self, Write};
    /// ```
    ///
    /// ## The Glob Operator
    ///
    /// If we want to bring *all* public items defined in a path into scope,
    /// we can specify that path followed by a `*`, the glob operator:
    /// ```rust
    /// use std::collections::*;
    /// ```
    ///
    /// The glob operator is often used when testing to bring everything under test
    /// into the `tests` module. The glob operator is also sometimes used as part of
    /// the prelude pattern.
    ()
}
