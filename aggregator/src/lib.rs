// Traits: Defining Shared Behavior
// A *trait* defines functionality a particular type has and can share with other types.
// We can use traits to define shared behavior in an abstract way.
// We can use *trait bounds* to specify that a generic type can be any type that has
// certain behavior.
// Note: Traits are similar to a feature often called *interfaces* in other languages.

// ### Defining a Trait
// A type's behavior consists of the methods we can call on that type. Different types share
// the same behavior if we can call the same methods on all of those types.
// Trait definitions are a way to group method signatures together to define a set of behaviors
// necessary to accomplish some purpose.o

// pub trait Summary {
//     fn summarize(&self) -> String;
// We declare the method signatures that describe the behaviors of the types that implement
// this trait.

// After the method signature, instead of providing an implementation within curly brackets, we
// use a semicolon. Each type implementing this trait must provide its own custom behavior for
// the body of the method. The compiler will enforce that any type has the `Summary` trait
// will have the method `summarize` defined with this signature exactly.
// }
// Here, we declare a trait using the `trait` keyword and then the trait's name, which is `Summary`
// in this case. We've also declared the trait as `pub` so that crates depending on this crate
// can make use of this trait too.

use std::fmt::{Debug, Display};
use std::iter::Sum;

// ### Implementing a Trait on a Type
#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// One restriction: we can implement a trait on a type only if at least one of the trait
// or the type is local to our crate!
// We cannot implement external traits on external types.
// This restriction is part of a property called *coherence*, and more specifically the
// *orphan rule*, so named because the parent type is not present.
// Without the rule, two crates could implement the same trait for the same type, and
// Rust wouldn't know which implementation to use.

// Default Implementations
// We can keep or override each method's default behavior.
// pub trait Summary {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
// }

// To use a default implementation, we specify an empty `impl` block.
// impl Summary for NewsArticle {}

// The syntax for overriding a default implementation is the same as the syntax for
// implementing a trait method that doesn't have a default implementation.

// Default implementations can call other methods in the same trait, even if those other methods
// don't have a default implementation. In this way, a trait can provide a lot of useful
// functionality and only require implementors to specify a small part of it.
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// To use this version of `Summary`, we only need to define `summarize_author` when we implement
// the trait on a type.
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Trait as Parameters
// We can use traits to define functions that accept many different types.
// To do this, we use the `impl Trait` syntax.
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }
// This parameter accepts any type that implements the specified trait.

// Trait Bound Syntax
// The `impl Trait` syntax works for straightforward cases but is actually
// syntax sugar for a longer form known as a *trait bound*.
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// The `impl Trait` syntax is convenient and makes for more concise code in simple
// cases, while the fuller trait bound syntax can express more complexity in other
// cases.
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
// If we want to force both parameters to have the same type, however, we must use a trait
// bound:
// pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// Specifying Multiple Trait Bounds with the `+` Syntax
// We can also specify more than one trait bound.
// pub fn notify(item: &(impl Summary + Display)) {}
// The `+` syntax is also valid with trait bounds on generic types:
// pub fn notify<T: Summary + Display>(item: &T) {}
// With the two trait bounds specified, the body of `notify` can call `summarize`
// and use `{}` to format `item`.

// Clearer Trait Bounds with `where` Clauses
// Each generic has its own trait bounds, so functions with multiple generic type parameters
// can contain lots of trait bound information between the function's name and its parameter
// list, making the function signature hard to read.
// For this reason, Rust has alternate syntax for specifying trait bounds inside a `where` clause
// after the function signature.
// So instead of writing this:
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// We can use a `where` clause, like this:
// fn some_function<T, U>(t: &T, u: &U) -> i32 where T: Display + Clone, U: Clone + Debug {
//     0
// }

// Returning Types that Implement Traits
// pub fn returns_summarizable() -> impl Summary {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you probably already know, people"
//         ),
//         reply: false,
//         retweet: false,
//     }
// }
// The ability to specify a return type only by the trait it implements is
// especially useful in the context of closures and iterators.
// Closures and iterators create types that only the compiler knows or types
// that are very long to specify.
// The `impl Trait` syntax lets us concisely specify that a function returns some
// type that implements the `Iterator` trait without needing to write out a very
// long type.

// WARNING: We can only use `impl Trait` if we're returning a single type.