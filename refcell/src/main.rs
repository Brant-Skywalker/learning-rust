#![allow(unused_doc_comments)]

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    /// ## `RefCell<T>` and the Interior Mutability Pattern
    ///
    /// *Interior mutability* is a design pattern in Rust that allows us to mutate
    /// data even when there are immutable references to that data; normally, this
    /// action is disallowed by the borrowing rules. To mutate data, the pattern
    /// uses `unsafe` code inside a data structure to bend Rust's usual rules that
    /// govern mutation and borrowing.
    ///
    /// We can use types that use the interior mutability pattern only when we can
    /// ensure that the borrowing rules will be followed at runtime, even though
    /// the compiler can't guarantee that. The `unsafe` code involved is then wrapped
    /// in a safe API, and the outer type is still immutable.

    /// ### Enforcing Borrowing Rules at Runtime with `RefCell<T>`
    /// Unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the data
    /// it holds. So, what makes `RefCell<T>` different from a type like `Box<T>`?
    ///
    /// With references and `Box<T>`, the borrowing rules' invariants are enforced at
    /// compile time. With `RefCell<T>`, these invariants are enforced at *runtime*.
    /// With references, if we break these rules, we'll get a compile error. With
    /// `RefCell<T>`, if we break these rules, our program will panic and exit.
    ///
    /// The advantages of checking the borrowing rules at compile time are that errors
    /// will be caught sooner in the development process, and there is no impact on
    /// runtime performance because all the analysis is completed **beforehand**.
    ///
    /// The advantage of checking the borrowing rules at runtime instead is that
    /// certain memory-safe scenarios are then allowed, where they would've been
    /// disallowed by the compile-time checks. Static analysis, like the Rust compiler,
    /// is inherently conservative. Some properties of code are impossible to detect
    /// by analyzing the code.
    ///
    /// The `RefCell<T>` type is useful when we're sure our code follows the borrowing
    /// rules but the compiler is unable to understand and guarantee that.
    ///
    /// Similar to `Rc<T>`, `RefCell<T>` is only for use in single-threaded scenarios
    /// and will give us a compile-time error if we try using it in a multithreaded
    /// context.
    ///
    /// Here is a recap of the reasons to choose `Box<T>`, `Rc<T>`, or `RefCell<T>`:
    /// * `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>`
    /// have single owners.
    /// * `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>`
    /// allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable
    /// or mutable borrows checked at runtime.
    /// * Because `RefCell<T>` allows mutable borrows checked at runtime, we **can mutate**
    /// the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.
    ///
    /// Mutating the value inside an immutable value is the *interior mutability* pattern.
    ///
    /// ### Interior Mutability: A Mutable Borrow to an Immutable Value
    ///
    /// A consequence of the borrowing rules is that when we have an immutable value,
    /// we can't borrow it mutably.
    /// ```rust
    /// let x = 5;
    /// let y = &mut x;  // Won't compile.
    /// ```
    ///
    /// However, there are situations in which it would be useful for value to mutate itself
    /// in its methods but appear immutable to other code. Code outside the value's methods
    /// would not be able to mutate the value. Using `RefCell<T>` is one way to get the ability
    /// to have interior mutability, but `RefCell<T>` doesn't get around the borrowing rules
    /// completely: the borrow checker in the compiler allows this interior mutability, and the
    /// borrowing rules are checked at runtime instead. If we violate the rules, we'll get a
    /// `panic!` instead of a compiler error.
    ///
    /// ### A Use Case for Interior Mutability: Mock Objects
    ///
    /// Sometimes during a programmer will use a type in place of another type, in order to
    /// observe particular behavior and assert it's implemented correctly. This placeholder
    /// type is called a *test double*.
    /// Test doubles stand in for other types when we're running tests.
    /// *Mock objects* are specific types of test doubles that record what happens during a
    /// test so we can assert that the correct actions took place.
    ///
    /// #### Keeping Track of Borrows at Runtime with `RefCell<T>`
    ///
    /// When creating immutable and mutable references, we use the `&` and `&mut` syntax,
    /// respectively. With `RefCell<T>`, we use the `borrow` and `borrow_mut` methods, which
    /// are part of the safe API that belongs to `RefCell<T>`. The `borrow` method returns
    /// the smart pointer type `Ref<T>`, and `borrow_mut` returns the smart pointer type
    /// `RefMut<T>`. Both types implement `Deref`, so we can treat them like regular references.
    ///
    /// The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are
    /// currently active. Every time we call `borrow`, the `RefCell<T>` increases its count of
    /// how many immutable borrows are active. When a `Ref<T>` value goes out of scope, the
    /// count of immutable borrows goes down by one. Just like the compile-time borrowing
    /// rules, `RefCell<T>` lets us have many immutable borrows or one mutable borrow at any
    /// point in time.
    ///
    /// If we try to violate these rules, rather than getting a compiler error as we would
    /// with references, the implementation of `RefCell<T>` will panic at runtime.
    ///
    /// ### Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`
    ///
    /// If we have an `Rc<T>` that holds a `RefCell<T>`, we can get a value that can have
    /// multiple owners *and* that we can mutate!
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};