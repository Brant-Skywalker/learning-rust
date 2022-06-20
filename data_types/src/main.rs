use std::io;

fn main() {
    /// We look at two data type subsets: scalar and compound.
    /// Rust is a *statically typed* language.
    /// In cases when many types are possible, we must add a type annotation.
    let guess: u32 = "42".parse().expect("Not a number");

    /// ## Scalar Types
    /// A scalar type represents a single value. Rust has four primary scalar types: integers,
    /// floating-point numbers, Booleans, and characters.

    /// ### Integer Types
    /// An integer is a number without a fractional component. Unsigned integer types start
    /// with `u`, and signed integer types start with `i`. Stored in 2's complement representation.
    /// Additionally, the `isize` and `usize` types depend on the architecture of the computer the
    /// program is running on: 64 bits if we're on a 64-bit architecture and 32-bit if we're on
    /// a 32-bit architecture.
    /// Note: number literals that can be multiple numeric types allow a type suffix, such as
    /// `57u8`, to designate the type. Number literals can also use `_` as a visual separator to
    /// make the number easier to read, such as `1_000`.
    /// Rust's **default**: `i32`.

    /// To explicitly handle the possibility of overflow:
    /// * Wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`.
    /// * Return the `None` value if there is overflow with the `checked_*` methods.
    /// * Return the value and a boolean indicating whether there was overflow with `overflow_*` methods.
    /// * Saturate at the value's minimum or maximum values with `saturating_*` methods.

    /// ### Floating-Point Types
    /// Rust's two floating-point types are `f32` and `f64`, which are 32 bits and 64 bits in size,
    /// respectively. **The default type is `f64`.**
    let x = 2.0; // f64. Represented according to the IEEE-754 standard.
    let y: f32 = 3.0; // f32.

    /// ### Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Integer division!

    // remainder.
    let remainder = 43 % 5;

    /// ### The Boolean Type
    let t = true;
    let f: bool = false; // With explicit type annotation.

    /// ### The Character Type
    /// Rust's `char` type is the language's most primitive alphabetic type.
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    let chinese = '傻';
    /// Note that we specify `char` literals with single quotes.
    /// Rust's `char` type is **four bytes in size** and represents a **Unicode Scalar Value**.
    /// Unicode Scalar Values range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive.

    /// ### Compound Types
    /// *Compound types can group multiple values into one type. Rust has two primitive compound
    /// types: tuples and arrays.

    /// **The Tuple Type**
    /// A tuple is a general way of grouping together a number of values **with a variety of
    /// types** into one compound type. Tuples have a fixed length: once declared, they cannot grow
    /// or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1); // With optional type annotations.

    /// To get the individual values out of a tuple, we can use pattern matching to destructure
    /// a tuple value:
    let (x, y, z) = tup; // This is called destructuring.
    println!("The value of y is: {}", y);

    /// We can also access a tuple element directly by using a period (`.`) followed by the index.
    let x: (i32, f64, u8) = (500, 6.4, 1); // With optional type annotations.
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    /// The tuple without any values, `()`, is a special type that has only one value, also written
    /// `()`. The type is called the *unit type* and the value is called the *unit value*.
    /// Expressions implicitly return the unit type if they don't return any other value.

    /// **The Array Type**
    /// Unlike a tuple, every element of an array must have the same size.
    /// Arrays in Rust have a fixed length.
    let a = [1, 2, 3, 4, 5];
    /// Arrays are more useful than vectors when we know the number of elements will not need to
    /// change.
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    /// We write an array's type using square brackets with the type of each elements, a semicolon,
    /// and then the number of elements in the array.
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    /// We can also initialize an array to contain the same value for each element by specifying
    /// the initial value, followed by a semicolon, and then the length of the array in square
    /// brackets.
    let a = [3; 5]; // Three fives.

    /// Accessing Array Elements
    /// An array is a single chunk of memory of a known, fixed size that can be allocated on the
    /// stack.
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    /// Invalid Array Element Access
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index]; // Rust checks that the index we've specified is less than the array length.
                            // If the index is greater than or equal to the length, Rust will panic.

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
