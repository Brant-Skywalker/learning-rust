fn main() {
    // *Smart pointers* are data structures that act like a pointer but also have
    // additional metadata and capabilities.
    // A *reference counting* smart pointer enables us to allow data to have multiple
    // owners by keeping track of the number of owners and, when no owners remain,
    // cleaning up the data.

    // Rust, with its concept of ownership and borrowing, has an additional
    // difference between references and smart pointers: while references only borrow data,
    // in many cases, smart pointers *own* the data they point to.

    // `String` and `Vec<T>` are smart pointers because they own some memory and allow us to
    // manipulate it.

    // Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart
    // pointers implement the `Deref` and `Drop` traits.

    // The `Deref` trait allows an instance of the smart pointer struct to behave like a
    // reference so we can write our code to work with either references or smart pointers.
    // The `Drop` trait allows us to customize the code that's run when an instance of the
    // smart pointer goes out of scope.

    // Most common smart pointers in the standard library:
    // `Box<T>` for allocating values on the heap.
    // `Rc<T>`, a reference counting type that enables **multiple ownership**
    // `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the
    // borrowing rules at runtime instead of compile time

    // The *interior mutability* pattern: an immutable type exposes an API for mutating
    // an interior value.
}
