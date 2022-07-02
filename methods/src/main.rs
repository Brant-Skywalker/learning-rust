#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// We start an `impl` (implementation) block for `Rectangle`. Everything within
//
impl Rectangle {
    // The `&self` here is actually short for `self: &Self`. Within an `impl` block,
    // the type `Self` is an alias for the type that the `impl` block is for. Methods
    // must have a parameter named `self` of type `Self` for their first parameter.
    fn area(&self) -> u32 {  // This method *borrows* the `Self` instance.
        self.width * self.height
    }
    // Methods can take ownership of `self`, borrow `self` immutably, or borrow
    // `self` mutably.
    // If we wanted to change the instance that we've called the method on as part of
    // what the method does, we'd use `&mut self` as the first parameter.

    // Having a method that takes ownership of the instance by using just `self` as the
    // first parameter is rare; this technique is usually used when the method transform
    // `self` into something else and we want to prevent the caller from using the original
    // instance after the transformation.

    // We can choose to give a method the same name as one of the struct's fields.
    fn width(&self) -> bool {
        self.width > 0
    }

    // Methods can take multiple parameters that we add to the signature after the `self`
    // parameter, and those parameters work just like parameters in functions.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // *Methods* are similar to functions: we declare them with the `fn` keyword
    // and a name.
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()  // Method syntax.
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // Rust doesn't have an equivalent to the `->` operator in C++. Rust automatically
    // adds in `&`, `&mut`, or `*` so `object` matches the signature of the method.

    // Methods with More Parameters
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Associated Functions
    // All functions defined within an `impl` block are called *associated functions* because
    // they're associated with the type named after `impl`.
    // One example: the `String::from` function that's defined on the `String` type.

    // Associated functions that aren't methods are often used for constructors that will return
    // a new instance of the struct.
    // To call this associated function, we use the `::` syntax with the struct name; `let sq =
    // Rectangle::square(3);` is an example.
    let sq = Rectangle::square(3);
    println!("{:?}", sq);

    // Multiple `impl` Blocks
    // Each struct is allowed to have multiple `impl` blocks.
}