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

pub trait Summary {
    fn summarize(&self) -> String;
    // We declare the method signatures that describe the behaviors of the types that implement
    // this trait.

    // After the method signature, instead of providing an implementation within curly brackets, we
    // use a semicolon. Each type implementing this trait must provide its own custom behavior for
    // the body of the method. The compiler will enforce that any type has the `Summary` trait
    // will have the method `summarize` defined with this signature exactly.
}
// Here, we declare a trait using the `trait` keyword and then the trait's name, which is `Summary`
// in this case. We've also declared the trait as `pub` so that crates depending on this crate
// can make use of this trait too.

// ### Implementing a Trait on a Type
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
