#![allow(unused_doc_comments)]

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Won't compile.
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn main() {
    /// # Paths for Referring to an Item in the Module Tree
    ///
    /// A path can take two forms:
    /// * An *absolute path* starts from a crate root by using a crate name (for
    /// code from an external crate or a literal `crate` (for code from the current
    /// crate).
    /// * A *relative path* starts from the current module and uses `self`,
    /// `super`, or an identifier in the current module.
    ///
    /// Both absolute and relative paths are followed by one or more identifiers
    /// separated by double colons (`::`).
    /// ```rust
    /// mod front_of_house {
    ///     mod hosting {
    ///         fn add_to_waitlist() {}
    ///     }
    /// }
    ///
    /// pub fn eat_at_restaurant() {
    ///     // Absolute path
    ///     crate::front_of_house::hosting::add_to_waitlist();
    ///
    ///     // Relative path
    ///     front_of_house::hosting::add_to_waitlist();
    /// }
    /// ```
    /// Using the `crate` name to start from the crate root is like using `/`
    /// to start from the filesystem root in our shell.
    ///
    /// Our preference is to specify absolute paths because it's more likely
    /// we'll want to move code definitions and item calls independently of
    /// each other.
    ///
    /// Modules also define Rust's *privacy boundary*: the line that encapsulates
    /// the implementation details external code isn't allowed to know about, call,
    /// or rely on. So if we want to make an item like a function or struct private
    /// we put it in a module.
    ///
    /// The way privacy works in Rust is that all items (functions, methods, structs,
    /// enums, modules, and constants) are private by default. Items in a parent module
    /// can't use the private items inside child modules, but items in child modules can
    /// use the items in their ancestor modules.
    ///
    /// ## Exposing Paths with the `pub` Keyword
    ///
    /// WARNING: Making a module public doesn't make its contents public.
    /// ```rust
    /// pub fn eat_at_restaurant() {
    ///     // Absolute path
    ///     crate::front_of_house::hosting::add_to_waitlist();
    ///
    ///     // Relative path
    ///     front_of_house::hosting::add_to_waitlist();
    /// }
    /// ```
    /// ### Best Practices for Packages with a Binary and a Library
    ///
    /// The module tree should be defined in `src/lib.rs`. Then, any public items
    /// can be used in the binary crate by starting paths with the name of package.
    /// The binary crate becomes a user of the library crate just like a completely external
    /// crate would use the library crate: it can only use the public API. This helps us
    /// design a good API; not only we are the authors, we're also clients!
    ///
    /// ## Starting Relative Paths with `super`
    ///
    /// We can also construct relative paths that begin in the parent module by using
    /// `super` at the start of the path. This is like starting a filesystem path with
    /// the `..` syntax.
    /// ```rust
    /// fn deliver_order() {}
    ///
    /// mod back_of_house {
    ///     fn fix_incorrect_order() {
    ///         cook_order();
    ///         super::deliver_order();
    ///     }
    ///
    ///     fn cook_order() {}
    /// }
    /// ```
    ///
    /// ## Making Structs and Enums Public
    ///
    /// We can also use `pub` to designate structs and enums as public, but there are
    /// a few extra details. If we use `pub` before a struct definition, we make the struct
    /// public, but the struct's fields will still be private.
    /// ```rust
    /// eat_at_restaurant();
    /// ```
    ///
    /// In contrast, if we make an enum public, all of its variants are then public.
    /// We only need the `pub` keyword before the `enum` keyword.
    ()
}
