fn main() {
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }

    // Enum Values
    // We can create instances of each of the two variants of `IpAddrKind` like this:
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // The variants of the enum are namespaced under its identifier, and we use a double
    // colon to separate the two. This is useful because now both values `IpAddrKind:V4`
    // and `IpAddrKind:V6` are of the same type: `IpAddrKind`.
    // fn route(ip_kind: IpAddrKind) {}
    //
    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    // We can use a struct to bundle the `kind` and `address` values together,
    // so now the variant is associated with the value.
    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }
    //
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    //
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // Representing the same concept using just an enum is more concise: rather than an enum
    // inside a struct, we can put data directly into each enum variant. This new definition of the
    // `IpAddr` enum says that both `V4` and `V6` variants will have associated `String` values:
    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }
    //
    // let home = IpAddr::V4(String::from("127.0.0.1"));  // Constructs an instance of `IpAddr`.
    // let loopback = IpAddr::V6(String::from("::1"));

    // We attach data to each variant of enum directly, so there is no need for an extra struct.

    // The name of each enum variant that we define also becomes a function that takes a `String`
    // argument and returns an instance of the `IpAddr` type. We automatically get this constructor
    // function defined as a result of defining the enum.

    // Another advantage of using an enum rather than a struct: each variant can have different
    // types and amounts of associated data.
    // enum IpAddr {
    //     V4(u8, u8, u8, u8),
    //     V6(String),
    // }
    //
    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));

    // How the standard library defines `IpAddr`: it has the exact enum and variants that we've
    // defined and used, but it embeds the address data inside the variants in the form of two
    // different structs, which are defined differently for each variant.
    struct Ipv4Addr {
        // --snip--
    }

    struct Ipv6Addr {
        // --snip--
    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    enum Message {
        Quit,
        // Has no data associated with it at all.
        Move { x: i32, y: i32 },
        // Has named fields like a struct does.
        Write(String),
        // Includes a single `String`.
        ChangeColor(i32, i32, i32),  // Includes three `i32` values.
    }

    // We're also able to define methods on enums.
    impl Message {
        fn call(&self) {
            // Method body would be defined here.
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // The `Option` Enum and Its Advantages Over Null Values
    // The `Option` Type encodes the very common scenario in which a value could be something
    // or it could be nothing.
    // Rust doesn't have the null feature that many other languages have. *Null* is a value
    // that means there is no value there. In languages with null, variables can always be in one
    // of two states: null or not-null.

    // Rust does have an enum that can encode the concept of a value being present or absent.
    // This enum is `Option<T>`.
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    // The `Option<T>` enum is so useful that it's even included in the preclude.
    // Its variants are also included in the preclude: we can use `Some` and `None`
    // directly without the `Option::` prefix.

    // The `<T>` is a generic type parameter.
    let some_number = Some(5);  // `Option<i32>`.
    let some_string = Some("a string"); // `Option<&str>`.

    let absent_number: Option<i32> = None;  // Type annotation required.

    // Note: `Option<T>` and `T` are different types.
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;  // Invalid!

    // Only when we have an `Option<i8>` do we have to worry about possibly not having a value,
    // and the compiler will make sure we handle that case before using that value.

    // Eliminating the risk of incorrectly assuming a not-null value helps us to be more confident
    // in our code. This was a deliberate design decision for Rust to limit null's pervasiveness
    // and increase the safety of Rust code.

    // The `match` expression is a control flow construct that does just this when used with enums:
    // it will run different code depending on which variant of the enum it has, and that code can
    // use the data inside the matching value.
}
