#![allow(unused_doc_comments)]

fn main() {
    /// # Characteristics of Object-Oriented Languages
    ///
    /// Arguably, OOP languages share certain common characteristics, namely objects,
    /// encapsulation, and inheritance.
    ///
    /// ## Objects Contain Data and Behavior
    ///
    /// Rust is object oriented: structs and enums have data, and `impl` blocks provide
    /// methods on structs and enums. Even though structs and enums with methods aren't
    /// *called* objects they provide the same functionality.
    ///
    /// ## Encapsulation that Hides Implementation Details
    ///
    /// Another aspect commonly associated with OOP is the idea of *encapsulation*, which
    /// means that the implementation details of an object aren't accessible to code using
    /// that object. Therefore, the only way to interact with an object is through its public
    /// API; code using the object shouldn't be able to reach into the object's internals and
    /// change data or behavior directly. This enables the programmer to change and refactor an
    /// object's internals without needing to change the code that uses the objects.
    ///
    /// We can use the `pub` keyword to decide which modules, types, functions, and methods in
    /// our code should be public, and by default everything else is private.
    ///
    /// If encapsulation is a required aspect for a language to be considered object oriented,
    /// then Rust meets that requirement. The option to use `pub` or not for different parts of
    /// code enables encapsulation of implementation details.
    ///
    /// ## Inheritance as a Type System and as Code Sharing
    ///
    /// *Inheritance* is a mechanism whereby an object can inherit from another object's definition,
    /// thus gaining the parent object's data and behavior without we having to define them again.
    ///
    /// The other reason to use inheritance relates to the type system: to enable a child type to
    /// be used in the same places as the parent type. This is also called *polymorphism*, which
    /// means that we can substitute multiple objects for each other at runtime if they share certain
    /// characteristics.
    ///
    /// Rust uses generics to abstract over different possible types and trait bounds to impose
    /// constraints on what those types must provide. This is sometimes called *bounded parametric
    /// polymorphism*.
    ///
    /// Inheritance has recently fallen out of favor as a programming design solution in many
    /// programming languages because it's often at risk of sharing more code than necessary.
    ///
    /// Rust takes a different approach, using trait objects instead of inheritance.
    ()
}

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    /// Private method!
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}