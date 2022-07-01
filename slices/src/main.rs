fn main() {
    // *Slices* let us reference a contiguous sequence of elements in a collection
    // rather than the whole collection.
    // A slice is a kind of reference, so it does not have ownership.

    // We could return the index of the end of the word, indicated by a space.
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word);
    s.clear();
    // `word` still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. `word` is now totally invalid!

    // ### String Slices
    // A *string slice* is a reference to part of a `String`.
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    // Rather than a reference to the entire `String`, `hello` is a reference to
    // a portion of the `String`, specified in the extra `[0..5]` bit. We create slices
    // using a range within brackets by specifying `[starting_index..ending_index]`,
    // where `starting_index` is the first position in the slice and `ending_index` is one
    // more than the last position in the slice.
    // Internally, the slice data structure stores the starting position and the **length** of
    // the slice, which corresponds to `ending_index` minus `starting_index`.

    // In the case of `let world = &s[6..11];`, `world` would be a slice that contains pointer
    // to the byte at index 6 of `s` with a length value of 5.

    // With Rust's `..` range syntax, if we want to start at index zero, we can drop the value
    // before the two periods.
    // They equivalent:
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];

    // By the same token, if our slice includes the last byte of the `String`, we can drop the
    // trailing number.
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    // We can also drop both values to take a slice of the entire string.
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];

    // WARNING: String slice range indices must occur at valid UTF-8 character boundaries.
    // If we attempt to create a string slice in the middle of a multibyte character, our program
    // will exit with an error.
    let mut s = String::from("hello world");
    let word = first_word(&s);
    // s.clear();  // It needs to get a mutable reference.
    println!("the first word is: {}", word);  // An immutable reference existing at the same time.

    // A more experienced Rustacean would write this signature instead because it allows us to use
    // the same function on both `&String` values and `&str` values.
    // fn first_word(s: &str) -> &str {

    // If we have a string slice, we can pass that directly. If we have a `String`, we can pass
    // a slice of the `String` or a reference to the `String`. This flexibility takes advantage of
    // *deref coercions*.
    // Defining a function to take a string slice instead of a reference to a `String` makes our
    // API more general and useful without losing any functionality.
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax.
    let word = first_word(my_string_literal);
    println!("{}", word);

    // #### Other Slices
    // There's a more general slice type. Consider this array:
    let a = [1, 2, 3, 4, 5];
    // We can refer to part of this array:
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    // This slice has the type `&[i32]`. It works the same way as string slices do,
    // by storing a reference to the first element and a length.
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();  // Convert our `String` to an array of bytes.
//
//     // Now we create an iterator over the array of bytes using the `iter` method.
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }

// Let's rewrite `first_word` to return a slice. The type that signifies "string slice"
// is written as `&str`.
// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
