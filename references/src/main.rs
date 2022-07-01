fn main() {
    // let s1 = String::from("hello");
    // let (s2, len) = calculate_length(s1);
    // println!("The length of '{}' is {}.", s2, len);

    // The issue here is that we have to return the `String` to the calling
    // so we can still use the `String` after the function call.
    // Instead, we can provide a reference to the `String` value.
    // A *reference* is like a pointer in that it's an address we can follow to
    // access the data stored at that address; that data is owned by some other
    // variable. Unlike a pointer, a reference is guaranteed to point to a valid
    // value of a particular type for the life of that reference.

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // The opposite of referencing by using `&` is *dereferencing*, which is
    // accomplished with the dereference operator, `*`.

    // We call the action of creating a reference *borrowing*.

    // Just as variables are immutable by default, so are references.
    // We are not allowed to modify something we have a reference to.
    // let s = String::from("hello");
    // change(&s);

    // ### Mutable References
    let mut s = String::from("hello");
    change(&mut s);  // Passing a mutable reference.

    // **One big restriction** of mutable references:
    // if we have a mutable reference to a value, we can have no other references
    // to that value.
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;  // Invalid!
    // println!("{}, {}", r1, r2);

    // The benefit of having this restriction is that Rust can prevent data
    // races at compile time. A *data race* is similar to a race condition
    // and causes undefined behavior.

    // We can use curly brackets to create a new scope, allowing for multiple mutable
    // references, just not *simultaneous* ones:
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{}", r1);
    }
    let r2 = &mut s;
    println!("{}", r2);

    // We also cannot have a mutable reference while we have an immutable one
    // to the same value.
    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s; // BIG PROBLEM!
    // println!("{}, {}, and {}", r1, r2, r3);

    // Use of an immutable reference don't expect the value to suddenly change
    // out from under them! However, multiple immutable references are allowed
    // because no one who is just reading the data has the ability to affect
    // anyone else's reading of the data.
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point.

    let r3 = &mut s;  // No problem.
    println!("{}", r3);

    // The ability of the compiler to tell that a reference is no longer being
    // used at a point before the end of the scope is called *Non-Lexical Lifetimes*
    // (NLL for short).

    // Rust compiler guarantees that references will never be dangling references:
    // if we have a reference to some data, the compiler will ensure that the data
    // will not go out of scope before the reference to the data does.
    // let reference_to_nothing = dangle();
}

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String
//     (s, length)
// }

// The ampersands represent *references*, and they allow us to refer to some
// value without taking ownership of it.
fn calculate_length(s: &String) -> usize {
    s.len()
}  // `s` goes out of scope here. But because it does not have ownership of what
// it refers to, it is not dropped.

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }