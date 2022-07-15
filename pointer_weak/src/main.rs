use crate::List::{Cons, Nil};
use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    /// # Reference Cycles Can Leak Memory
    ///
    /// Rust allows memory leaks by using `Rc<T>` and `RefCell<T>`:
    /// it's possible to create references where items refer to each other
    /// in a cycle.
    ///
    /// ## Creating a Reference Cycle
    ///
    /// We are adding a `tail` method to make it convenient for us to access the
    /// second item if we have a `Cons` variant.
    ///
    /// This code creates a reference cycle.
    ///
    /// ```rust
    /// let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    ///
    /// println!("a initial rc count = {}", Rc::strong_count(&a));
    /// println!("a next item = {:?}", a.tail());
    ///
    /// let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    ///
    /// println!("a rc count after b creation = {}", Rc::strong_count(&a));
    /// println!("b initial rc count = {}", Rc::strong_count(&b));
    /// println!("b next item = {:?}", b.tail());
    ///
    /// if let Some(link) = a.tail() {
    ///     *link.borrow_mut() = Rc::clone(&b);
    /// }
    ///
    /// println!("b rc count after changing a = {}", Rc::strong_count(&b));
    /// println!("a rc count after changing a = {}", Rc::strong_count(&a));
    ///
    /// // This will overflow the stack.
    /// println!("a next item = {:?}", a.tail());
    /// ```
    ///
    /// ## Preventing Reference Cycles: Turning an `Rc<T>` into a `Weak<T>`
    ///
    /// We can also create a *weak reference* to the value within an `Rc<T>`
    /// instance by calling `Rc::downgrade` and passing a reference to `Rc<T>`.
    /// Strong references are how we can share ownership of an `Rc<T>` instance.
    /// Weak references don't express an ownership relationship, and their count
    /// doesn't affect when an `Rc<T>` instance is cleaned up. They won't cause
    /// a reference cycle because any cycle involving some weak references will
    /// be broken once the strong reference count of value involved is 0.
    ///
    /// When we call `Rc::downgrade`, we get a smart pointer of type `Weak<T>`.
    /// Instead of increasing the `strong_count` in the `Rc<T>` instance by 1,
    /// calling `Rc::downgrade` increases the `weak_count` by 1. The `Rc<T>` type
    /// uses `weak_count` to keep track of how many `Weak<T>` references exist,
    /// similar to `strong_count`. The difference is the `weak_count` doesn't need
    /// to be 0 for the `Rc<T>` instance to be cleaned up.
    ///
    /// Because the value that `Weak<T>` references might have been dropped, to do
    /// anything with the value that a `Weak<T>` is pointing to, we must make sure
    /// the value still exists. Do this by calling the `upgrade` method on a `Weak<T>`
    /// instance, which will return an `Option<Rc<T>>`. We'll get a result of `Some`
    /// if the `Rc<T>` value has not been dropped yet and a result of `None` if the
    /// `Rc<T>` value has been dropped. Because `upgrade` returns an `Option<Rc<T>>`,
    /// Rust will ensure that the `Some` case and the `None` case are handled, and
    /// there won't be an invalid pointer.
    ///
    /// ### Creating a Tree Data Structure: a `Node` with Child Nodes
    ///
    /// ```rust
    /// #[derive(Debug)]
    /// struct Node {
    ///     value: i32,
    ///     children: RefCell<Vec<Rc<Node>>>,
    /// }
    /// let leaf = Rc::new(Node {
    ///     value: 3,
    ///     children: RefCell::new(vec![]),
    /// });
    ///
    /// let branch = Rc::new(Node {
    ///     value: 5,
    ///     children: RefCell::new(vec![Rc::clone(&leaf)]),
    /// });
    /// ```
    ///
    /// ### Adding a Reference from a Child to Its Parent
    ///
    /// The `parent` field can't contain an `Rc<T>`, because that would create
    /// a reference cycle with `leaf.parent` pointing to `branch` and `branch.children`
    /// pointing to `leaf`, which would cause their `strong_count` values to never be 0.
    ///
    /// Instead of `Rc<T>`, we'll make the type of `parent` use `Weak<T>`,
    /// specifically a `RefCell<Weak<Node>>`.
    ///
    /// ```rust
    /// let leaf = Rc::new(Node {
    ///     value: 3,
    ///     parent: RefCell::new(Weak::new()),
    ///     children: RefCell::new(vec![]),
    /// });
    ///
    /// println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    ///
    /// let branch = Rc::new(Node {
    ///     value: 5,
    ///     parent: RefCell::new(Weak::new()),
    ///     children: RefCell::new(vec![Rc::clone(&leaf)]),
    /// });
    ///
    /// *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    ///
    /// println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    /// ```
    /// We use the `borrow_mut` method on the `RefCell<Weak<Node>>` in the parent
    /// field of `leaf`, and then we use the `Rc::downgrade` function to create a
    /// `Weak<Node>` reference to `branch` from the `Rc<Node>` in `branch`.
    ///
    /// ### Visualizing Changes to `strong_count` and `weak_count`
    ///
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
