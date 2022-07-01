// Rust *does* include functionality to print out debugging information.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // A program that calculate the area of a rectangle.

    // let width1 = 30;
    // let height1 = 50;
    //
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    // ### Refactoring with Tuples
    // let rect1 = (30, 50);
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect1)
    // );

    // ### Refactoring with Structs: Adding More Meaning
    // We use structs to add meaning by labelling the data.
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // ### Adding Useful Functionality with Derived Traits
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // println!("rect1 is {}", rect1);  // `std::fmt::Display` not implemented.
    // println!("rect1 is {:?}", rect1);  // `Debug` not implemented. Add `#[derived(Debug)]`.
    println!("rect is {:#?}", rect1);  // Pretty-print.

    // Another way to print out a value using the `Debug` format is to use the `dbg!` macro,
    // which takes ownership of an expression, prints the file and line number of where that
    // `dbg!` macro call occurs in our code along with the resulting value of that expression,
    // and returns ownership of the value.
    // Note: Calling the `dbg` macro prints to the standard error console stream (`stderr`), as
    // opposed to `println!` which prints to the standard output console stream (`stdout`).
    let scale = 2;
    let mut rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
    rect1 = dbg!(rect1);  // Returns ownership.
    dbg!(rect1);

    // We can put `dbg!` around the expression `30 * scale` and, because `dbg!` returns ownership of
    // the expression's value, the `width` field will get the same value as if we didn't have the
    // `dbg!` call there.
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Now defined with one parameter, whose type is an immutable borrow of a struct
// `Rectangle` instance.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
