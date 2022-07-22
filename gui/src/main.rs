#![allow(unused_doc_comments)]

use gui::{Draw, Button, Screen};

fn main() {
    /// # Using Trait Objects That Allow for Values of Different Types
    ///
    /// ## Defining a Trait for Common Behavior
    ///
    /// To implement the behavior we want `gui` to have, we'll define a trait named
    /// `Draw` that will have one method named `draw`. Then we can define a vector
    /// that takes a *trait object*. A trait object points to both an instance of a type
    /// implementing our specified trait as well as a table used to look up trait methods
    /// on that type at runtime. We create a trait object by specifying some sort of pointer,
    /// such as a `&` reference or a `Box<T>` smart pointer, then the `dyn` keyword, and then
    /// specifying the relevant trait. We can use trait objects in place of a generic or
    /// concrete type. Wherever we use a trait object, Rust's type system wil ensure at compile time
    /// that any value used in that context will implement the trait object's trait.
    /// Consequently, we don't need to know all the possible know all the possible types at compile
    /// time.
    ///
    /// Trait objects *are* more like objects in other languages in the sense that they combine
    /// data and behavior. But trait objects differ from traditional objects in that we cannot add
    /// data to a trait object. Trait objects aren't as generally useful as objects in other
    /// languages: their specific purpose is to allow abstraction across common behavior.
    ///
    /// This works differently from defining a struct that uses a generic type parameter with trait
    /// bounds. A generic type parameter can only be substituted with one concrete type at a time,
    /// whereas trait objects allow for multiple concrete types to fill in for the trait object at
    /// runtime.
    ///
    /// If we'll only ever have homogeneous collections, using generics and trait bounds is preferable
    /// because the definitions will be monomorphized at compile time to use the concrete types.
    ///
    /// On the other hand, with the method using trait objects, one `Screen` instance can hold a
    /// `Vec<T>` that contains a `Box<Button>` as well as a `Box<TextField>`.
    ///
    /// ## Implementing the Trait
    ///
    /// Now we'll add some types that implement the `Draw` trait.
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ]
    };

    screen.run();

    /// The advantage of using trait objects and Rust's type system to write code similar
    /// to code using duck typing is that we never have to check whether a value implements
    /// a particular method at runtime or worry about getting errors if a value doesn't implement
    /// a method but we call it anyway. Rust won't compile our code if the values don't implement
    /// the traits that the trait objects need.
    /// ```rust
    /// let screen = Screen {
    ///     components: vec![Box::new(String::from("Hi"))]  // Compiler error!
    /// };
    ///
    /// screen.run();
    /// ```
    ///
    /// ## Trait Objects Perform Dynamic Dispatch
    ///
    /// The code that results from monomorphization is doing *static dispatch*, which is when the
    /// compiler knows what method we're calling at compile time. This is opposed to *dynamic
    /// dispatch*, which is when the compiler cannot tell at compile time which method we're calling.
    /// In dynamic dispatch cases, the compiler emits code that at runtime will figure out which
    /// method to call.
    ///
    /// When we use trait objects, Rust must use dynamic dispatch. The compiler doesn't know all the
    /// types that might be used with the code that is using trait objects, so it doesn't know which
    /// method implemented on which type to call. Instead, at runtime, Rust uses the pointers inside
    /// the trait object to know which method to call. There is a runtime cost when this lookup
    /// happens that doesn't occur with static dispatch. Dynamic dispatch also prevents the compiler
    /// from choosing to inline a method's code, which in turn prevents some optimizations. However,
    /// we did get extra flexibility, so it's a trade-off to consider.
    ()
}

/// If someone using our library decides to implement a `SelectBox` struct:
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
