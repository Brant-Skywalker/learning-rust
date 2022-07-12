// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;
// Not in the prelude.
use crate::List::{Cons, Nil};


fn main() {
    /// ## `Rc<T>`, the Reference Counted Smart Pointer
    ///
    /// There are cases when a single value might have multiple owners.
    ///
    /// We have to enable multiple ownership explicitly by using the Rust type
    /// `Rc<T>`, which is an abbreviation for *reference counting*. The `Rc<T>`
    /// type keeps track of the number of references to a value to determine
    /// whether or not the value is still in use. If there are zero references
    /// to a value, the value can be cleaned up without any references becoming
    /// invalid.
    ///
    /// We use the `Rc<T>` type when we want to allocate some data on the heap
    /// for multiple parts of our program to read and we **can't determine at
    /// compile time** which part will finish using the data last.
    ///
    /// *NOTE: `Rc<T>` is only for use in single-threaded scenarios!*

    /// ### Using `Rc<T>` to Share Data
    /// ```rust
    /// let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    /// let b = Cons(3, Box::new(a));
    /// let c = Cons(4, Box::new(a));  // Compile error!
    /// ```
    ///
    /// The variants own the data they hold, so when we create the `b` list, `a`
    /// is moved into `b` and `b` owns `a`. Then, when we try to use `a` again when
    /// creating `c`, we're not allowed to because `a` has been moved.
    ///
    /// We'll change our definition of `List` to use `Rc<T>` in place of `Box<T>`.
    /// When we create `b`, instead of taking ownership of `a`, we'll clone the
    /// `Rc<List>` that `a` is holding, thereby increasing the number of references
    /// from one to two and letting `a` and `b` share ownership of the data in that
    /// `Rc<List>`.
    ///
    /// Every time we call `Rc::clone`, the reference count to the data within the
    /// `Rc<List>` will increase, and the data won't be cleaned up unless there are
    /// zero references to it.
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    /// We could have called `a.clone()` rather than `Rc::clone(&a)`, but Rust's
    /// convention is to use `Rc::clone` in this case. The implementation of
    /// `Rc::clone` **doesn't make a deep copy** of all the data copy of all the data
    /// like most types' implementations of `clone` do. The call to `Rc::clone` only
    /// increments the reference count, which doesn't take much time.
    /// By using `Rc::clone` for reference counting, we can visually distinguish
    /// between the deep-copy kinds of clones and the kinds of clones that increase
    /// the reference count.
    /// When looking for performance problems in the code, we only need to consider
    /// the deep-copy clones and can disregard calls to `Rc::clone`.

    /// ### Cloning an `Rc<T>` Increases the Reference Count
    /// ```rust
    /// let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    /// println!("count after creating a = {}", Rc::strong_count(&a));
    /// let b = Cons(3, Rc::clone(&a));
    /// println!("count after creating b = {}", Rc::strong_count(&a));
    /// {
    ///     let c = Cons(4, Rc::clone(&a));
    ///     println!("count after creating c = {}", Rc::strong_count(&a));
    /// }
    /// println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    /// ```
    /// We print the reference count by calling the `Rc::strong_count` function.
    ///
    /// We don't have to call a function to decrease the reference count like we
    /// have to call `Rc::clone` to increase the reference count: the implementation
    /// of the `Drop` trait decreases the reference count automatically when a `Rc<T>`
    /// value goes out of scope.
    ///
    /// Using `Rc<T>` allows a single value to have multiple owners, and the count
    /// ensures that the value remains valid as long as any of the owners still
    /// exist.
    ///
    /// Via immutable references, `Rc<T>` allows us to share data between multiple
    /// parts of your program for reading only.
    ()
}


