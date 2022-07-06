fn main() {
    // ## Generic Data Types

    // In Function Definitions
    // let number_list = vec![34, 50, 25, 100, 65];
    //
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);
    //
    // let char_list = vec!['y', 'm', 'a', 'q'];
    //
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    // In Struct Definitions
    // We can also define structs to use a generic type parameter in one or more
    // fields using the `<>` syntax.
    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 };

    // Multiple generic type parameters:
    // let both_integer = Point { x: 5, y: 10 };
    // let both_float = Point { x: 1.0, y: 4.0 };
    // let integer_and_float = Point { x: 5, y: 4.0 };

    // In Enum Definitions
    // We can define enums to hold generic data types in their variants.
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // Enums can use multiple generic types as well.
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // In Method Definitions
    // let p = Point { x: 5, y: 10 };
    // println!("p.x = {}", p.x());

    // mixed-up
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Rust performs monomorphization of the code using generics at compile time.
    // We pay no runtime cost for using generics.
}

// To define the generic `largest` function, place type name declarations inside
// angle brackets, `<>`, between the name of the function and the parameter list.
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//
//     for &item in list {
//         if item > largest {  // Doesn't compile for now.
//             largest = item;
//         }
//     }
//
//     largest
// }
// The function `largest` is generic over some type `T`. This function has one parameter
// named `list`, which is a slice of values of type `T`. The `largest` function will return
// a value of the same type `T`.

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

// We can implement methods on structs and enums and use generic types in their definitions, too.
// Note: we have to declare `T` just after `impl` so we can use `T` to specify that we're
// implementing methods on the type `Point<T>`. By declaring `T` as a generic type after `impl`,
// Rust can identify that the type in the angle brackets in `Point` is a generic type rather than a
// concrete type.
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// We can also specify constraints on generic types when defining methods on the type.
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2).sqrt())
//     }
// }

// Generic type parameters in a struct definition aren't always the same as those we use in that
// same struct's method signatures.

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}