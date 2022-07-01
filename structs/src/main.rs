fn main() {
    // Structs are more flexible than tuples.
    // Inside curly brackets, we define the names and types of the pieces of data,
    // which we call *fields*.
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // We create an *instance* of a struct by specifying concrete values for each of
    // the fields.
    // We create an instance by stating the name of the struct and then add curly
    // brackets containing `key: value` pairs, where the keys are the names of the fields
    // and the values are the data we want to store in those fields.
    // We don't have to specify the fields in the same order in which we declared them
    // in the struct.
    // In other words, the struct definition is like a general template for the type,
    // and instances fill in that template with particular data to create values of the type.
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Use dot notation to get a specific value from a struct.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    // Note that the entire instance must be mutable; Rust doesn't allow us to mark only
    // certain fields as mutable.

    // As with any expression, we can construct a new instance of the struct as the last
    // expression in the function body to implicitly return that new instance.
    fn build_user(email: String, username: String) -> User {
        User {
            email,  // The `email` field and the `email` parameter have the same name.
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // ### Creating Instances From Other Instances With Struct Update Syntax
    // We can create a new instance of a struct that includes most of the values from
    // another instance but changes some, using struct *update syntax*.
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // The syntax `..` specifies that the remaining fields not explicitly set should have
    // the same value as the fields in the given instance.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
        // Must come last to specify that any remaining fields should get their values from
        // the corresponding fields in `user1`.
    };
    // We can no longer use `user1` after creating `user2` because the `String` in the `username`
    // field of `user1` was moved into `user2`.
    user1.email = String::from("fuck");

    // ### Using Tuple Structs without Named Fields to Create Different Types
    // Rust also supports structs that look similar to tuples, called *tuple structs*.
    // Tuple structs have the added meaning the struct name provides but don't have names associated
    // with their fields; rather, they just have the types of the fields.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // Note that the `black` and `origin` values are different types, because they're instances of 
    // different tuple structs. Each struct we define is its own type, even though the fields within
    // the struct have the same types.
    // We can use a `.` followed by the index to access an individual value.

    // ### Unit-Like Structs Without Any Fields
    // We can also define structs that don't have any fields!
    // Unit-like structs can be useful when we need to implement a trait on some type but don't have
    // any data that we want to store in the type itself.
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}

