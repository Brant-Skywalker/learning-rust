fn main() {
    /// # Extensible Concurrency with `Sync` and `Send` Traits
    ///
    /// ## Allowing Transference of Ownership Between Threads with `Send`
    ///
    /// The `Send` marker trait indicates that ownership of values of the type implementing
    /// `Send` can be transferred between threads. Almost every Rust type is `Send`, but there
    /// are some exceptions, including `Rc<T>`.
    ///
    /// ## Allowing Access from Multiple Threads with `Sync`
    ///
    /// The `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be
    /// referenced from multiple threads. In other words, any type `T` is `Sync` if `&T` (an
    /// immutable reference to `T`) is `Send`, meaning the reference can be sent safely to another
    /// thread. Similar to `Send`, primitive types are `Sync`, and types composed entirely of
    /// types that are `Sync` are also `Sync`.
    ///
    /// The smart pointer `Rc<T>` is also not `Sync` for the same reasons that it's not `Send`.
    /// The `RefCell<T>` type and the family of related `Cell<T>` types are not `Sync`. The
    /// implementation of borrow checking that `RefCell<T>` does at runtime is not thread-safe.
    /// The smart pointer `Mutex<T>` is `Sync` and can be used to share access with multiple threads.
    ///
    /// ## Implementing `Send` and `Sync` Manually Is Unsafe
    ///
    /// Because types that are made up of `Send` and `Sync` traits are automatically also `Send` and
    /// `Sync`, we don't have to implement those traits manually. As marker traits, they don't even
    /// have any methods to implement. They're just useful for enforcing invariants related to
    /// concurrency.
    ///
    /// Manually implementing these traits involves implementing unsafe Rust code.
}
