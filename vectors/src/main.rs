fn main() {
    // Vectors allow us to store more than one value in a single data structure
    // that puts all the values next to each other in memory.
    // Vectors can only store values of the same type.

    // Creating a New Vector
    // To create a new empty vector, we call the `Vec::new` function.
    let v: Vec<i32> = Vec::new();
    // We need to add a type annotation here.
    // Vectors are implemented using generics.

    // More often, we create a `Vec<T>` with initial values and Rust will infer
    // the type of value we want to store.
    // Rust conveniently provides the `vec!` macro, which will create a new vector
    // that holds the values we give it.
    let v = vec![1, 2, 3];

    // Because we've given the initial `i32` values, Rust can infer that the type of `v`
    // is `Vec<i32>`, and the type annotation isn't necessary.

    // Updating a Vector
    // To create a vector and then add elements to it, we can use the `push` method.
    let mut v = Vec::new();  // No annotation needed.
    v.push(5);  // Rust infers the type `Vec<i32>` from the data.
    v.push(6);
    v.push(7);
    v.push(8);

    // As with any variable, if we want to be able to change its value, we need to
    // make it mutable using the `mut` keyword.

    // Dropping a Vector Drops Its Elements
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    }  // <- v goes out of scope and is freed here.

    // When the vector gets dropped, all of its contents are also dropped, meaning
    // those integers it holds will be cleaned up.

    // Reading Elements of Vectors
    // Two ways to reference a value stored in a vector: via indexing or using the `get` method.
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];  // This gives us a reference.
    println!("The third element is {}", third);

    match v.get(2) {  // Type `Option<&T>`.
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.")
    }

    // The reason Rust provides these two ways to reference an element is so we
    // can choose how the program behaves when we try to use an index value outside the
    // range of existing elements.
    // let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];  // Panics!
    // let does_not_exist = v.get(100);
    // The first `[]` method will cause the program to panic because it references a nonexistent
    // element. This method is best used when we want our program to crash if there's an attempt
    // to access an element past the end of the vector.

    // When the `get` method is passed an index that is outside the vector, it returns `None`
    // without panicking. We would use this method if accessing an element beyond the range of the
    // vector may happen occasionally under normal circumstances.

    // When the program has a valid reference, the borrow checker enforces the ownership and
    // borrowing rules to ensure this reference and any other references to the contents of the
    // vector remain valid.
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {}", first);  // Error!
    // This error is due to the way vectors work: because vectors put the values next to each other
    // in memory and copying the old elements to the new space, if there isn't enough room to put
    // all the elements next to each other where the vector is currently stored.
    // In that case, the reference to the first element would be pointing to deallocated memory!
    // The borrowing rules prevent programs from ending up in that situation.

    // Iterating over the Values in a Vector
    // To access each element in a vector in turn, we would iterate through all of the elements
    // rather than use indices to access one at a time.
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // We can also iterate over mutable reference to each element in a mutable vector in order to
    // make changes to all elements.
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    // To change the value that the mutable reference refers to, we have to use the `*` dereference
    // operator to get to the value in `i` before we can use the `+=` operator.

    // Using an Enum to Store Multiple Types
    // The variants of an enum are defined under the same enum type, so when we need one type to
    // represent elements of different types, we can define and use an enum!

    // We can define an enum whose variants will hold the different type values, and all the enum
    // variants will be considered the same type: that of the enum. Then we can create a vector to
    // hold that enum and so, ultimately, holds different types.
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Rust needs to know what types will be in the vector at compile time so it knows exactly
    // how much memory on the heap will be needed to store each element. We must also be explicit
    // about what types are allowed in this vector.
    // Using an enum plus a `match` expression means that Rust will ensure at compile time that
    // every possible case is handled.
    println!("{:?}", row);

    // If we don't know the exhaustive set of types a program will get at runtime to store in a
    // vector, use a trait object instead.
}

