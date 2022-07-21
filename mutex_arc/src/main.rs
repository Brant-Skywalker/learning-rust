#![allow(unused_doc_comments)]

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    /// # Shared-State Concurrency
    ///
    /// Shared memory concurrency is like multiple ownership: multiple threads can access
    /// the same memory location at the same time.
    ///
    /// ## Using Mutexes to Allow Access to Data from One Thread at a Time
    ///
    /// *Mutex* is an abbreviation for *mutual exclusion*, as in, a mutex allows only one thread to
    /// access some data at any given time. To access the data in a mutex, a thread must first
    /// signal that it wants access by asking to acquire the mutex's *lock*. The lock is a data
    /// structure that is part of the mutex that keeps track of who currently has exclusive access
    /// to the data. Therefore, the mutex is described as *guarding* the data it holds via the
    /// locking system.
    ///
    /// Mutexes have a reputation for being difficult to use because we have to remember two rules:
    /// * We must attempt to acquire the lock before using the data.
    /// * When we're done with the data that the mutex guards, we must unlock the data so other
    /// threads can acquire the lock.
    ///
    /// ### The API of `Mutex<T>`
    ///
    /// ```rust
    /// let m = Mutex::new(5);
    ///
    /// {
    ///     let mut num = m.lock().unwrap();
    ///     *num = 6;
    /// }
    ///
    /// println!("m = {:?}", m);
    /// ```
    ///
    /// As with many types, we create a `Mutex<T>` using the associated function `new`. To access the
    /// data inside the mutex, we use the `lock` method to acquire the lock. This call will block
    /// the current thread so it can't do any work until it's our turn to have the lock.
    ///
    /// The call to `lock` would fail if another thread holding the lock panicked. In that case,
    /// no one would ever be able to get the lock, so we've chosen to `unwrap` and have this thread
    /// panic if we're in that situation.
    ///
    /// After we've acquired the lock, we can treat the return value, named `num` in this case,
    /// as a mutable reference to the data inside. The type system ensures that we acquire a lock
    /// before using the value in `m`. The type of `m` is `Mutex<i32>`, not `i32`, so we *must* call
    /// `lock` to be able to use the `i32` value.
    ///
    /// `Mutex<T>` is a smart pointer. More accurately, the call to `lock` *returns* a smart pointer
    /// called `MutexGuard`, wrapped in a `LockResult` that we handled with the call to `unwrap`.
    /// The `MutexGuard` smart pointer implements `Deref` to point at our inner data; the smart
    /// pointer also has a `Drop` implementation that releases the lock automatically when a
    /// `MutexGuard` goes out of scope, which happens at the end of the inner scope. As a result,
    /// we don't risk forgetting to release the lock and blocking the mutex from being used by
    /// other threads, because the lock release happens automatically.
    ///
    /// ### Sharing a `Mutex<T>` Between Multiple Threads
    ///
    /// Now, let's try to share a value between multiple threads using `Mutex<T>`.
    /// ```rust
    /// let counter = Mutex::new(6);
    /// let mut handles = vec![];
    ///
    /// for _ in 0..10 {
    ///     let handle = thread::spawn(move || { // Compiler error.
    ///         let mut num = counter.lock().unwrap();
    ///
    ///         *num += 1;
    ///     });
    ///     handles.push(handle);
    /// }
    ///
    /// for handle in handles {
    ///     handle.join().unwrap();
    /// }
    ///
    /// println!("Result: {}", *counter.lock().unwrap());
    /// ```
    ///
    /// Rust is telling us that we can't move the ownership of lock `counter` into multiple threads.
    ///
    /// ### Multiple Ownership with Multiple Threads
    ///
    /// Now we wrap the `Mutex<T>` in `Rc<T>` and clone the `Rc<T>` before moving ownership to the
    /// thread.
    /// ```rust
    /// let counter = Rc::new(Mutex::new(0));
    /// let mut handles = vec![];
    ///
    /// for _ in 0..10 {
    ///     let counter = Rc::clone(&counter);
    ///     let handle = thread::spawn(move || {
    ///         let mut num = counter.lock().unwrap();
    ///
    ///         *num += 1;
    ///     });
    ///     handles.push(handle);
    /// }
    ///
    /// for handle in handles {
    ///     handle.join().unwrap();
    /// }
    ///
    /// println!("Result: {}", *counter.lock().unwrap());
    /// ```
    ///
    /// The trait `Send` is not implemented for `Rc<Mutex<i32>>.
    /// It's one of the traits that ensures the types we use with threads are meant for use in
    /// concurrent situations.
    ///
    /// Unfortunately, `Rc<T>` is not sage to share across threads. When `Rc<T>` manages the
    /// reference count, it adds to the count for each call to `clone` and subtracts from the count
    /// when each clone is dropped. But it doesn't use any concurrency primitives to make sure that
    /// changes to the count can't be interrupted by another thread. This could lead to wrong counts
    /// --subtle bugs that could in turn lead to memory leaks or a value being dropped before we're
    /// done with it. What we need is a type exactly like `Rc<T>` but one that makes changes to the
    /// reference count in a thread-safe way.
    ///
    /// ### Atomic Reference Counting with `Arc<T>`
    ///
    /// Fortunately, `Arc<T>` *is* a type like `Rc<T>` that is safe to use in concurrent situations.
    /// The *a* stands for *atomic*, meaning it's an *atomically reference counted* type. Atomics
    /// are an additional kind of concurrency primitive.
    ///
    /// We fix our program by changing the `use` line, the call to `new`, and the call to `clone`.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    /// ## Similarities Between `RefCell<T>`/`Rc<T>` and `Mutex<T>`/`Arc<T>`
    ///
    /// `Mutex<T>` also provides interior mutability, as the `Cell` family does.
    ///
    /// Another detail to note is that Rust cannot protect us from all kinds of logic errors
    /// when we use `Mutex<T>`. Using `Rc<T>` came with the risk of creating reference cycles, where
    /// two `Rc<T>` values refer to each other, causing memory leaks.
    ///
    /// Similarly, `Mutex<T>` comes with the risk of creating *deadlocks*. These occur when an operation
    /// needs to lock two resources and two threads have each acquired one of the locks, causing them
    /// to wait for each other forever.
    ()
}

