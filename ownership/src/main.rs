fn main() {
    // Ownership Rules
    // * Each value in Rust has a variable that's called its *owner*.
    // * There can only be one owner at a time.
    // * When the owner goes out of scope, the value will be dropped.

    // The `String` Type
    // This type manages data allocated on the heap and as such is able to
    // store an amount of text that is unknown to us at compile time.

    // We can create a `String` from a string literal using the `from`
    // function:
    let _s = String::from("hello");
    // The double colon `::` operator allows us to namespace this particular
    // `from` function under the `String` type rather than using some sort of
    // name like `string_from`.

    // This kind of string can be mutated:
    let mut s = String::from("hello");
    s.push_str(", world!");  // push_str() appends a literal to a String.
    println!("{}", s); // This will print `hello, world!`

    // * The memory must be requested from the memory allocator at runtime.
    // * We need a way of returning this memory to the allocator.

    // Rust takes a different path: the memory is automatically returned once
    // the variable that owns it goes out of scope.
    {
        let s = String::from("hello");

        // do stuff with s
    }  // Now Rust calls a special function `drop` for us to return the memory.
    // This pattern of deallocating resources at the end of an item's lifetime
    // is sometimes called *Resource Acquisition Is Initialization* (RAII) in C++.

    // ### Ways Variables and Data Interact: Move
    let s1 = String::from("hello");
    let s2 = s1;

    // A `String` is made up of three parts: a pointer to the memory that holds
    // the contents of the string, a length, and a capacity. This group of data
    // is stored on the stack.

    // The length is how much memory, in bytes, the contents of the `String` is
    // currently using. The capacity is the total amount of memory, in bytes, that
    // the `String` has received from the allocator.

    // When we assign `s1` to `s2`, the `String` data is copied, meaning that we
    // copy the pointer, the length, and the capacity that are on the stack. We do
    // not copy the data on the heap that the pointer refers to.

    // To ensure memory safety, after the line `let s2 = s1`, Rust considers `s1` as
    // no longer valid. Therefore, Rust doesn't need to free anything when `s1` goes out
    // of scope.
    // **`s1` was *moved* into `s2`!**
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // Compiler error: invalidated reference.

    // Design choice: Rust will never automatically create "deep" copies of our
    // data. Therefore, any automatic copying can be assumed to be inexpensive in
    // terms of runtime performance.

    // ### Ways Variables and Data Interact: Clone
    // If we *do* want to deeply copy the heap data of the `String`, not just
    // the stack data, we can use a common method called `clone`.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // ### Stack-Only Data: Copy
    // This is valid:
    let x = 5;
    let y = x; // Or `x.clone()`.
    println!("x = {}, y = {}", x, y);
    // Reason: Types such as integers that have a known size at compile time are
    // stored entirely on the stack, so copies of the actual values are quick to
    // make. There's no difference between deep and shallow copying here, so calling
    // `clone` wouldn't do anything different from the usual shallow copying and we
    // can leave it out.

    // Rust has a special annotation called the `Copy` trait that we can place on types
    // that are stored on the stack like integers are. If a type implements the `Copy`
    // trait, a variable is still valid after assignment to another variable. Rust won't
    // let us annotate a type with `Copy` if the type, or any of its parts, has implemented
    // the `Drop` trait. If the type needs something special to happen when the value goes out
    // of scope and we add the `Copy` annotation to that type, we'll get a compile-time error.

    // Tuples that only contain types that also implement `Copy` implement `Copy`.
    // For example, `(i32, i32)` implements `Copy`, but `(i32, String)` does not.

    // ## Ownership and Functions
    // The semantics for passing a value to a function are similar to those for assigning a
    // value to a variable. Passing a variable to a function will move or copy, just as assignment
    // does.
    let s = String::from("hello");  // `s` comes into scope.

    takes_ownership(s);  // `s`'s value moves into the function...
    // ... and so is no longer valid here.

    let x = 5;  // `x` comes into scope.

    makes_copy(x);  // `x` would move into the function,
    // but `i32` implements Copy, so it's okay to still use x afterwards.

    // println!("{}", s);  // Compile-time error!
    println!("{}", x);  // OK.

    // ### Return Values and Scope
    // Returning values can also transfer ownership.
    let s1 = gives_ownership();  // `gives_ownership` moves its return value into `s1`.

    let s2 = String::from("hello");

    // `s2` is moved into `takes_and_gives_back`, which also moves its return value into `s3`.
    let s3 = takes_and_gives_back(s2);

    // The ownership of a variable follows the same pattern every time: assigning a value
    // to another variable moves it. When a variable that includes data on the heap goes out of
    // scope, the value will be cleaned up by `drop` unless ownership of the data has been moved
    // to another variable.

    // Rust does let us return multiple values using a tuple.
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    // Rust has a feature for using a value without transferring ownership, called *references*.
}

fn takes_ownership(some_string: String) { // `some_string` comes into scope.
    println!("{}", some_string);
}  // Here, `some_string` goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {  // `some_integer` comes into scope.
    println!("{}", some_integer);
}  // Here, `some_integer` goes out of scope. Nothing special happens.

// This function moves its return value into the function that calls it.
fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string  // `some_string` is returned and moves out to the calling function.
}

// This function takes a String and returns one.
fn takes_and_gives_back(a_string: String) -> String {
    a_string  // `a_string` is returned and moves out to the calling function.
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();  // `len()` returns the length of a String.

    (s, length)
}