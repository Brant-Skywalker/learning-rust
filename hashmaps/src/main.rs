fn main() {
    // The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type
    // `V` using a *hashing function*.

    // Hash maps are useful when we want to look up data not by using an index,
    // but by using a key that can be of any type.

    // Creating a New Hash Map
    // One way to create an empty hash map is using `new` and adding elements with `insert`.
    // use std::collections::HashMap;
    //
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // We need to first `use` the `HashMap` from the collections portion of the standard library.
    // It is not included in the features brought into scope automatically in the prelude.

    // Hash maps store their data on the heap.
    // Hash maps are homogeneous: all the keys/values must have the same type.

    // Another way of constructing a hash map is by using iterators and the `collect` method on a
    // vector of tuples, where each tuple consists of a key and its value.
    // The `collect` method gathers data into a number of collection types, including `HashMap`.

    // We could use the `zip` method to create an iterator of tuples.
    // Then we could use the `collect` method to turn that iterator of tuples into a hash map.
    // use std::collections::HashMap;
    //
    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];
    // let mut scores: HashMap<_, _> =
    //     teams.into_iter().zip(initial_scores.into_iter()).collect();
    // The type annotation `HashMap<_, _>` is needed here because it's possible to `collect` into
    // many different data structures and Rust doesn't know which we want unless we specify.
    // For the parameters for the key and value types, however, we use underscores, and Rust can
    // infer the types that the hash map contains based on the types of the data in the vectors.

    // Hash Maps and Ownership
    // For types that implement the `Copy` trait, like `i32`, the values are copied into the hash
    // map. For owned values like `String`, the values will be moved and the hash map will be the
    // owner of those values.
    // use std::collections::HashMap;
    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");
    // 
    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    // `field_name` and `field_value` are invalid at this point.

    // If we insert references to values into the hash map, the values won't be moved into the hash
    // map. The values that the references point to must be valid for at least as long as the hash
    // map is valid.

    // Accessing Values in a Hash Map
    // use std::collections::HashMap;
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    //
    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);

    // Here, `score` will have the value that's associated with the Blue team, and the result will
    // be `Some(&10)`. The result is wrapped in `Some` because `get` returns an `Option<&V>`; if
    // there's no value for that key in the hash map, `get` will return `None`.
    // match score {
    //     Some(value) => println!("{}", value),
    //     None => ()
    // }

    // We can iterate over each key/value pair in a hash map.
    // use std::collections::HashMap;
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    //
    // for (key, value) in scores {
    //     println!("{}: {}", key, value);
    // }

    // Updating a Hash Map

    // Overwriting a Value
    // If we insert a key and a value into a hash map and then insert that same key with a
    // different value, the value associated with that key will be replaced.
    // use std::collections::HashMap;
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 50);
    // println!("{:?}", scores);

    // Only Inserting a Value If the Key Has No Value
    // Hash maps have a special API for this called `entry` that takes the key we want to check
    // as a parameter.
    // The return value of the `entry` method is an enum called `Entry` that represents a value
    // that might or might not exist.
    // use std::collections::HashMap;
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    //
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);
    //
    // println!("{:?}", scores);
    // The `or_insert` method on `Entry` is defined to return a mutable reference to the value for
    // the corresponding `Entry` key if that key exists, and if not, inserts the parameter as the
    // new value for this key and returns a mutable reference to the new value.

    // Updating a Value Based on the Old Value
    use std::collections::HashMap;
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;  // First dereference.
    }
    // The mutable reference goes out of scope at the end of the `for` loop.
    println!("{:?}", map);
    // The `split_whitespace` method iterates over sub-slices, separated by whitespace, of the
    // value in `text`. The `or_insert` method returns a mutable reference `&mut V` to the value
    // for the specified key.

    // By default, `HashMap` uses a hashing function called *SipHash* that can provide resistance
    // to DDoS attacks involving hash tables.
    // A *hasher* is a type that implements the `BuildHasher` trait.
}
