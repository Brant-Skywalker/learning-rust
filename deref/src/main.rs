#![allow(unused_doc_comments)]

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    /// ## Treating Smart Pointers Like Regular References with the `Deref` Trait
    ///
    /// Implementing the `Deref` trait allows us to customize the behavior of the
    /// *dereference operator* `*`. By implementing `Deref` in such a way that a
    /// smart pointer can be treated like a regular reference, we can write code that
    /// operates on references and use that code with smart pointers too.

    /// ### Following the Pointer to the Value
    ///
    /// A regular reference is a type of pointer, and one way to think of a pointer
    /// is as an arrow to a value stored somewhere else.
    /// ```rust
    /// let x = 5;
    /// let y = &x;
    ///
    /// assert_eq!(5, x);
    /// assert_eq!(5, *y);
    /// ```
    /// We have to use `*y` to follow the reference to the value it's pointing to so
    /// the compiler can compare the actual value.

    /// ### Using `Box<T>` Like a Reference
    ///
    /// ```rust
    /// let x = 5;
    /// let y = Box::new(x);
    ///
    /// assert_eq!(5, x);
    /// assert_eq!(5, *y);
    /// ```
    /// Here we set `y` to be an instance of a box pointing to a copied value of `x`
    /// rather than a reference pointing to the value of `x`.
    /// In the last assertion, we can use the dereference operator to follow the box's
    /// pointer in the same way that we did when `y` was a reference.

    /// ### Defining Our Own Smart Pointer
    ///
    /// Let's build a smart pointer similar to the `Box<T>` type provided by the standard
    /// library to experience how smart pointers behave differently from references by
    /// default. Then we'll look at how to add the ability to use the dereference operator.
    ///
    /// The `Box<T>` type is ultimately defined as a tuple struct with one element. We'll
    /// also define a `new` function to match the `new` function defined on `Box<T>`.
    /// ```rust
    /// struct MyBox<T> (T);  // A tuple struct.
    ///  
    /// impl<T> MyBox<T> {
    ///     fn new(x: T) -> MyBox<T> {
    ///         MyBox(x)
    ///     }
    /// }
    /// ```
    ///
    /// ```rust
    /// let x = 5;
    /// let y = MyBox::new(x);
    ///
    /// assert_eq!(5, x);
    /// assert_eq!(5, *y);
    /// ```
    /// Our `MyBox<T>` cannot be dereferenced because we haven't implemented that ability
    /// on our type. To enable dereferencing with the `*` operator, we implement the `Deref`
    /// trait.

    /// ### Treating a Type Like a Reference by Implementing the `Deref` Trait
    ///
    /// The `Deref` trait, provided by the standard library, requires us to implement one
    /// method named `deref` that borrows `self` and returns a reference to the inner data.
    /// ```rust
    /// use std::ops::Deref;
    ///
    /// impl<T> Deref for MyBox<T> {
    ///     type Target = T;
    ///
    ///     fn deref(&self) -> &Self::Target {
    ///         &self.0
    ///     }
    /// }
    /// ```
    /// The `type Target = T;` syntax defines an associated type for the `Deref` trait to use.
    /// Associated types are a slightly different way of declaring a generic parameter.
    ///
    /// The `deref` method gives the compiler the ability to take a value of any type that
    /// implements `Deref` and call the `deref` method to get a `&` reference that it knows how
    /// to dereference.
    ///
    /// When we entered `*y`, behind the scenes Rust actually ran this code:
    /// ```rust
    /// *(y.deref())
    /// ```
    ///
    /// Rust substitutes the `*` operator with a call to the `deref` method and then a plain
    /// dereference so we don't have to think about whether or not we need to call the `deref`
    /// method. This Rust feature lets us write code that functions identically whether we have
    /// a regular reference or a type that implements `Deref`.
    ///
    /// The reason the `deref` method returns a reference to a value, and that the plain
    /// dereference outside the parentheses in `*(y.deref())` is still necessary, is to do with
    /// the ownership system. If the `deref` method returned the value directly instead of a
    /// reference to the value, the value would be moved out of `self`. We don't want to take
    /// ownership of the inner value inside `MyBox<T>` in this case or in most cases where we
    /// use the dereference operator.

    /// ### Implicit Deref Coercions with Functions and Methods
    ///
    /// *Deref coercion* converts a **reference** to a type that implements the `Deref` trait into
    /// a reference to another type. For example, deref coercion can convert `&String` to `&str`
    /// because `String` implements the `Deref` trait such that it returns `&str`.
    ///
    /// Deref conversion is a convenience Rust performs on arguments to functions and methods,
    /// and works only on types that implement the `Deref` trait. It happens automatically when
    /// we pass a reference to a particular type's value as an argument to a function or method
    /// that doesn't match the parameter type in the function or method definition.
    /// A sequence of calls to the `deref` method converts the type we provided into the type the
    /// parameter needs.
    ///
    /// ```rust
    /// fn hello(name: &str) {
    ///     println!("Hello, {}", name);
    /// }
    ///
    /// let m = MyBox::new(String::from("Rust"));
    /// hello(&m);
    /// ```
    /// Deref coercion makes it possible to call `hello` with a reference to a value of type
    /// `MyBox<String>`.
    ///
    /// When the `Deref` trait is defined for the types involved, Rust will analyze the types and
    /// use `Deref::deref` as many times as necessary to get a reference to match the parameter's
    /// type. The number of times that `Deref::deref` needs to be inserted is resolved at compile
    /// time, so there is **no runtime penalty** for taking advantage of deref coercion!

    /// ### How Deref Coercion Interacts with Mutability
    ///
    /// We can use the `DerefMut` trait to override the `*` operator on mutable references.
    ///
    /// Rust does deref coercion when it finds types and trait implementations in three cases:
    /// * From `&T` to `&U` when `T: Deref<Target=U>`
    /// * From `&mut T` to `&mut U` when `T: Deref<Target=U>`
    /// * From `&mut T` to `U` when `T: Deref<Target=U>`
    ///
    /// The first case states that if we have a `&T`, and `T` implements `Deref` to some type `U`,
    /// we can get a `&U` transparently. The second case states that the same deref coercion happens
    /// for mutable references.
    ///
    /// The third case is trickier: Rust will also coerce a mutable reference to an immutable one.
    /// But the reverse is *not* possible: immutable references will never coerce to mutable
    /// references. Because of the borrowing rules, if we have a mutable reference, that mutable
    /// reference must be the only reference to that data. Converting one mutable reference to one
    /// immutable reference will never break the borrowing rules. Converting an immutable reference
    /// to a mutable reference would require that the initial immutable reference is the only immutable
    /// reference to that data, but the borrowing rules don't guarantee that.
    /// Therefore, Rust cannot make the assumption that converting an immutable reference to a
    /// mutable reference is possible.
}