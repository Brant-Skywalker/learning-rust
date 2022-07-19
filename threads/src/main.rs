#![allow(unused_doc_comments)]

use std::thread;
use std::time::Duration;

fn main() {
    /// # Using Threads to Run Code Simultaneously
    ///
    /// The Rust standard library uses one operating system thread per one language thread.
    /// There are crates that implement other models of threading that make different tradeoffs to
    /// the 1:1 model.
    ///
    /// ## Creating a New Thread with `spawn`
    ///
    /// To create a new thread, we call the `thread::spawn` function and pass it a closure
    /// containing the code we want to run in the new thread.
    ///
    /// ```rust
    /// thread::spawn(|| {
    ///     for i in 1..10 {
    ///         println!("hi number {} from the spawned thread!", i);
    ///         thread::sleep(Duration::from_millis(1));
    ///     }
    /// });
    ///
    /// for i in 1..5 {
    ///     println!("hi number {} from the main thread", i);
    ///     thread::sleep(Duration::from_millis(1));
    /// }
    /// ```
    ///
    /// Note that when the main thread of a Rust program completes, all spawned threads are shut
    /// down, whether or not they have finished running.
    ///
    /// The calls to `thread::sleep` force a thread to stop its execution for a short duration,
    /// allowing a different thread to run. The threads will probably take turns, but that isn't
    /// guaranteed: it depends on how our operating system schedules the threads. In this run,
    /// the main thread printed first, even though the print statement from the spawned thread
    /// appears first in the code.
    ///
    /// ## Waiting for All Threads to Finish Using `join` Handles
    ///
    /// We can fix the problem of the spawned thread not running or ending prematurely by saving
    /// the return value of `thread::spawn` in a variable. The return type of `thread::spawn` is
    /// `JoinHandle`. A `JoinHandle` is an owned value that, when we call the `join` method on it,
    /// will wait for its thread to finish.
    ///
    /// ```rust
    /// let handle = thread::spawn(|| {
    ///     for i in 1..10 {
    ///         println!("hi number {} from the spawned thread!", i);
    ///         thread::sleep(Duration::from_millis(1));
    ///     }
    /// });
    ///
    /// for i in 1..5 {
    ///     println!("hi number {} from the main thread!", i);
    ///     thread::sleep(Duration::from_millis(1));
    /// }
    ///
    /// handle.join().unwrap();
    /// ```
    ///
    /// Calling `join` on the handle blocks the thread currently running until the thread represented
    /// by the handle terminates. *Blocking* a thread means that thread is prevented from performing
    ///
    /// The two threads continue alternating, but the main thread waits because of the call to `handle.join()`
    /// and does not end until the spawned thread is finished.
    ///
    /// ```rust
    /// let handle = thread::spawn(|| {
    ///     for i in 1..10 {
    ///         println!("hi number {} from the spawned thread!", i);
    ///         thread::sleep(Duration::from_millis(1));
    ///     }
    /// });
    ///
    /// handle.join().unwrap();
    ///
    /// for i in 1..5 {
    ///     println!("hi number {} from the main thread!", i);
    ///     thread::sleep(Duration::from_millis(1));
    /// }
    /// ```
    ///
    /// Small details, such as where `join` is called, can affect whether or not our threads run
    /// at the same time.
    ///
    /// ## Using `move` Closures with Threads
    ///
    /// We'll often use the `move` keyword with closures passed to `thread::spawn` because the
    /// closure will then take ownership of the value it uses from the environment, thus transferring
    /// ownership of those values from one thread to another.
    ///
    /// ```rust
    /// let v = vec![1, 2, 3];
    ///
    /// let handle = thread::spawn(|| {
    ///     println!("Here's a vector: {:?}", v);  // Compiler error!
    /// });
    ///
    /// drop(v);
    ///
    /// handle.join().unwrap();
    /// ```
    ///
    /// Rust *infers* how to capture `v`, and because `println!` only needs a reference to `v`,
    /// the closure tries to borrow `v`. However, there's a problem: Rust can't tell how long the
    /// spawned thread will run, so it doesn't know if the reference to `v` will always be valid.
    ///
    /// To fix the compiler error, we can use the `move` keyword.
    /// ```rust
    /// let v = vec![1, 2, 3];
    ///
    /// let handle = thread::spawn(move || {
    ///     println!("Here's a vector: {:?}", v);
    /// });
    ///
    /// handle.join().unwrap();
    /// ```
    ///
    /// By adding the `move` keyword before the closure, we force the closure to take ownership
    /// of the values it's using rather than allowing Rust to infer that it should borrow the values.
    ///
    /// The `move` keyword overrides Rust's conservative default of borrowing; it doesn't let us
    /// violate the ownership rules.
}
