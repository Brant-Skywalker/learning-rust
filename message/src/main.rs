#![allow(unused_doc_comments)]

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    /// # Using Message Passing to Transfer Data Between Threads
    ///
    /// One increasingly popular approach to ensuring safe concurrency is *message passing*, where
    /// threads or actors communicate by sending each other messages containing data.
    ///
    /// Do not communicating by sharing memory; instead, share memory by communicating.
    ///
    /// To accomplish message-sending concurrency, Rust's standard library provides an implementation
    /// of *channels*. A channel is a general programming concept by which data is sent from one thread
    /// to another.
    ///
    /// A channel has two halves: a transmitter and a receiver. The transmitter half is the upstream
    /// location where we put rubber ducks into the river, and the receiver half is where the rubber
    /// duck ends up downstream. One part of our code calls methods on the transmitter with the data
    /// we want to send, and another part checks the receiving end for arriving messages. A channel
    /// is said to be *closed* if either the transmitter or receiver half is dropped.
    /// ```rust
    /// let (tx, rx) = mpsc::channel();
    /// ```
    ///
    /// We create a new channel using the `mpsc::channel` function; `mpsc` stands for *multiple producer,
    /// single consumer*. In short, the way Rust's standard library implements channels means a channel
    /// can have multiple *sending* ends that produce values but only one *receiving* end that
    /// consumes those values.
    ///
    /// The `mpsc::channel` function returns a tuple, the first element of which is the sending end
    /// --transmitter--and the second element is the receiving end--the receiver.
    /// The abbreviations `tx` and `rx` are traditionally used in many fields for *transmitter* and
    /// *receiver* respectively, so we name our variables as such to indicate each end. We're
    /// using a `let` statement with a pattern that destructures the tuples.
    ///
    /// Let's move the transmitting end into a spawned thread and have it send one string so
    /// the spawned thread is communicating with the main thread.
    /// ```rust
    /// let (tx, rx) = mpsc::channel();
    ///
    /// thread::spawn(move || {
    ///     let val = String::from("hi");
    ///     tx.send(val).unwrap();
    /// });
    /// ```
    ///
    /// Again, we're using `thread::spawn` to create a new thread and then using `move` to move `tx`
    /// into the closure so the spawned thread owns `tx`. The spawned thread needs to own the transmitter
    /// to be able to send messages through the channel. The transmitter has a `send` method that takes
    /// the value we want to send. The `send` method returns a `Result<T, E>` type, so if the receiver
    /// has already been dropped and there's nowhere to send a value, the send operation will return
    /// an error. In this example, we are calling `unwrap` to panic in case of an error.
    /// ```rust
    /// let (tx, rx) = mpsc::channel();
    ///
    /// thread::spawn(move || {
    ///     let val = String::from("hi");
    ///     tx.send(val).unwrap();
    /// });
    ///
    /// let received = rx.recv().unwrap();
    /// println!("Got: {}", received);
    /// ```
    ///
    /// The receiver has two useful methods: `recv` and `try_recv`. We're using `recv`, short for
    /// *receive*, which will block the main thread's execution and wait until a value is sent down
    /// the channel. Once a value is sent, `recv` will return it in a `Result<T, E>`. When the transmitter
    /// closes, `recv` will return an error to signal that no more values will be coming.
    ///
    /// The `try_recv` method doesn't block, but will instead return a `Result<T, E>` **immediately**:
    /// an `Ok` value holding a message if one is available and an `Err` value if there aren't any
    /// messages this time. Using `try_recv` is useful if this thread has other work to do while
    /// waiting for messages: we could write a loop that calls `try_recv` every so often, handles a
    /// message if one is available, and otherwise does other work for a little while until checking
    /// again.
    ///
    /// ## Channels and Ownership Transference
    ///
    /// The ownership rules play a vital role in message sending because they help us write
    /// safe, concurrent code.
    ///
    /// Now we try to use a `val` value in the spawned thread *after* we've sent it down the channel.
    /// ```rust
    /// let (tx, rx) = mpsc::channel();
    ///
    /// thread::spawn(move || {
    ///     let val = String::from("hi");
    ///     tx.send(val).unwrap();
    ///     println!("val is {}", val);  // Compiler error!
    /// });
    ///
    /// let received = rx.recv().unwrap();
    /// println!("Got: {}", received);
    /// ```
    ///
    /// ## Sending Multiple Values and Seeing the Receiver Waiting
    ///
    /// The spawned thread will now send multiple messages and pause for a second between each
    /// message.
    /// ```rust
    /// let (tx, rx) = mpsc::channel();
    ///
    /// thread::spawn(move || {
    ///     let vals = vec![
    ///         String::from("hi"),
    ///         String::from("from"),
    ///         String::from("the"),
    ///         String::from("thread"),
    ///     ];
    ///
    ///     for val in vals {
    ///         tx.send(val).unwrap();
    ///         thread::sleep(Duration::from_secs(1));
    ///     }
    /// });
    ///
    /// for received in rx {
    ///     println!("Got: {}", received);
    /// }
    /// ```
    ///
    /// In the main thread, we're not calling the `recv` function explicitly anymore: instead,
    /// we're treating `rx` as an iterator. For each value received, we're printing it. When
    /// the channel is closed, iteration will end.
    ///
    /// ## Creating Multiple Producers by Cloning the Transmitter
    ///
    /// Now let's put `mpsc` to create multiple threads that all send values to the same receiver.
    ///
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
