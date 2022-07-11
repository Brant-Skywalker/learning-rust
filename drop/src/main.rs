#![allow(unused_doc_comments)]

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    /// ## Running Code on Cleanup with the `Drop` Trait
    ///
    /// The second trait important to the smart pointer pattern is `Drop`, which
    /// lets you customize what happens when a value is about to go out of scope.

    /// We can provide an implementation for the `Drop` trait on any type, and the
    /// code can be used to release resources like files or network connections.
    ///
    /// When a `Box<T>` is dropped it will deallocate the space on the heap that
    /// the box points to.
    ///
    /// We can specify the code to run when a value goes out of scope by implementing
    /// the `Drop` trait. The `Drop` trait requires us to implement one method named
    /// `drop` that takes a mutable reference to `self`. To see when Rust calls `drop`,
    /// let's implement `drop` with `println!` statements for now.
    ///
    /// ```rust
    /// let c = CustomSmartPointer {
    ///     data: String::from("my stuff"),
    /// };
    /// let d = CustomSmartPointer {
    ///     data: String::from("other stuff"),
    /// };
    /// println!("CustomSmartPointers created.");
    /// ```
    /// Rust automatically called `drop` for us when our instances went out of scope,
    /// calling the code we specified. Variables are dropped in the reverse order of their
    /// creation, so `d` was dropped before `c`.

    /// ### Dropping a Value Early with `std::mem::drop`
    ///
    /// Occasionally, we might want to clean up a value early.
    /// One example is when using smart pointers that manage locks: we might want to force
    /// the `drop` method that releases the lock so that other code in the same scope can
    /// acquire the lock. We have to call the `std::mem::drop` function provided by the
    /// standard library if we want to force a value to be dropped before the end of its scope.
    ///
    /// ```rust
    /// let c = CustomSmartPointer {
    ///     data: String::from("some data"),
    /// };
    /// println!("CustomSmartPointer created.");
    /// c.drop();
    /// println!("CustomSmartPointer dropped before the end of main.");
    /// ```
    /// Rust doesn't let us call drop explicitly because Rust would still automatically call
    /// `drop` on the value at the end of `main`. This would cause a *double free* error because
    /// Rust would be trying to clean up the same value twice.
    ///
    /// The `std::mem::drop` function is different from the `drop` method in the `Drop` trait.
    /// We call it by passing as an argument the value we want to force stop. The function is in
    /// the **prelude**!
    /// ```rust
    /// let c = CustomSmartPointer {
    ///     data: String::from("some data"),
    /// };
    /// println!("CustomSmartPointer created.");
    /// drop(c);
    /// println!("CustomSmartPointer dropped before the end of main.");
    /// ```
    /// We can use code specified in a `drop` trait implementation in many ways to make cleanup
    /// convenient and safe. With the `Drop` trait and Rust's ownership system, we don't have to
    /// remember to clean up because Rust does it automatically.
    ()
}
