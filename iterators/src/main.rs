#![allow(unused_doc_comments)]

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
    /// # Processing a Series of Items with Iterators
    ///
    /// The iterator pattern allows us to perform some task on a sequence of items in turn.
    /// An iterator is responsible for the logic of iterating over each item and determining
    /// when the sequence has finished.
    ///
    /// In Rust, iterators are *lazy*, meaning they have no effect until we call methods that
    /// consume the iterator to use it up.
    /// ```rust
    /// let v1 = vec![1, 2, 3];
    /// let v1_iter = v1.iter();
    /// ```
    ///
    /// Once we've created an iterator, we can use it in a variety of ways.
    /// ```rust
    /// let v1 = vec![1, 2, 3];
    /// let v1_iter = v1.iter();
    /// for val in v1_iter {
    ///     println!("Got: {}", val);
    /// }
    /// ```
    /// The iterator is stored in the `v1_iter` variable, and no iteration takes place at that
    /// time. When the `for` loop is called using the iterator in `v1_iter`, each element in the
    /// iterator is used in one iteration of the loop, which prints out each value.
    ///
    /// Iterators give us more flexibility to use the same logic with many different kinds of
    /// sequences.
    ///
    /// ## The `Iterator` Trait and the `next` Method
    ///
    /// All iterators implement a trait named `Iterator` that is defined in the standard library.
    /// ```rust
    /// pub trait Iterator {
    ///     type Item;
    ///
    ///     fn next(&mut self) -> Option<Self::Item>;
    ///
    ///     // methods with default implementations elided
    /// }
    /// ```
    ///
    /// Notice this definition uses some new syntax: `type Item` and `Self::Item`, which are
    /// defining an *associated type* with this trait. The `Item` type will be the type returned
    /// from the iterator.
    ///
    /// The `Iterator` trait only requires implementors to define one method: the `next` method,
    /// which returns one item of the iterator at a time wrapped in `Some` and, when the iteration
    /// is over, returns `None`.
    ///
    /// We can call the `next` method on iterators directly.
    /// ```rust
    /// let v1 = vec![1, 2, 3];
    /// let mut v1_iter = v1.iter();
    /// assert_eq!(v1_iter.next(), Some(&1));
    /// assert_eq!(v1_iter.next(), Some(&2));
    /// assert_eq!(v1_iter.next(), Some(&3));
    /// assert_eq!(v1_iter.next(), None);
    /// ```
    ///
    /// We need to make `v1_iter` mutable: calling the `next` method on an iterator changes internal
    /// state that the iterator uses to keep track of where it is in the sequence. In other words,
    /// this code *consumes*, or uses up, the iterator. Each call to `next` eats up an item
    /// from the iterator. We didn't need to make `v1_iter` mutable when we used a `for` loop
    /// because the loop took ownership of `v1_iter` and made it mutable behind the scenes.
    ///
    /// Also note that the values we get from the calls to `next` are immutable references to the
    /// values in the vector. The `iter` method produces an iterator over immutable references. If
    /// we want to create an iterator that takes ownership of `v1` and returns owned values, we
    /// can call `into_iter` instead of `iter`. Similarly, if we want to iterate over mutable
    /// references, we can call `iter_mut` instead of `iter`.
    ///
    /// ## Methods that Consumes the Iterator
    ///
    /// The `Iterator` trait has a number of different methods with default implementations provided
    /// by the standard library. Some of these methods call the `next` method in their definition,
    /// which is why we're required to implement the `next` method when implementing the `Iterator`
    /// trait.
    ///
    /// Methods that call `next` are called *consuming adaptors*, because calling them uses up the
    /// iterator.
    /// ```rust
    /// let v1 = vec![1, 2, 3];
    /// let v1_iter = v1.iter();
    /// let total: i32 = v1_iter.sum();
    /// assert_eq!(total, 6);
    /// ```
    /// We aren't allowed to use `v1_iter` after the call to `sum` because `sum` takes ownership
    /// of the iterator we call it on.
    ///
    /// ## Methods that Produces Other Iterators
    ///
    /// Other methods defined on the `Iterator` trait, known as *iterator adaptors*, allow us to
    /// change iterators into different kinds of iterators. We can chain multiple calls to iterator
    /// adaptors to perform complex actions in a readable way. But because all iterators are lazy,
    /// we have to call one of the consuming adaptor methods to get the results from calls to
    /// iterator adaptors.
    /// ```rust
    /// let v1: Vec<i32> = vec![1, 2, 3];
    /// v1.iter().map(|x| x + 1);  // Warning! Iterators are lazy.
    /// ```
    ///
    /// To fix this, we'll use the `collect` method. This method consumes the iterator and
    /// collects the resulting values into a collection data type.
    /// ```rust
    /// let v1: Vec<i32> = vec![1, 2, 3];
    /// let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    /// assert_eq!(v2, vec![2, 3, 4]);
    /// ```
    /// Because `map` takes a closure, we can specify any operation we want to perform on each
    /// item. This is a great example of how closures let you customize some behavior while reusing
    /// the iteration behavior that the `Iterator` trait provides.
    ///
    /// Here we collect the results of iterating over the iterator that's returned from the call to
    /// `map` into a vector. This vector will end up containing each item from the original vector
    /// incremented by 1.
    ///
    /// ## Using Closures that Capture Their Environment
    ///
    /// The `filter` method on an iterator takes a closure that takes each item from the iterator
    /// and returns a Boolean. If the closure returns `true`, the value will be included in the
    /// iterator produced by `filter`. If the closure returns `false`, the value won't be included
    /// in the resulting iterator.
    /// ```rust
    /// let shoes = vec![
    ///     Shoe {
    ///         size: 10,
    ///         style: String::from("sneaker"),
    ///     },
    ///     Shoe {
    ///         size: 13,
    ///         style: String::from("sandal"),
    ///     },
    ///     Shoe {
    ///         size: 10,
    ///         style: String::from("boot"),
    ///     },
    /// ];
    ///
    /// let in_my_size = shoes_in_size(shoes, 10);
    /// assert_eq!(
    ///     in_my_size,
    ///     vec![
    ///         Shoe {
    ///             size: 10,
    ///             style: String::from("sneaker"),
    ///         },
    ///         Shoe {
    ///             size: 10,
    ///             style: String::from("boot"),
    ///         },
    ///     ]
    /// );
    /// ```
    ///
    /// The `shoes_in_size` function takes ownership of a vector of shoes and a shoe size as
    /// parameters. It returns a vector containing only shoes of the specified size.
    ///
    /// In the body of `shoes_in_size`, we call `into_iter` to create an iterator that takes
    /// ownership of the vector. Then we call `filter` to adapt that iterator into a new iterator
    /// that only contains elements for which the closure returns `true`.
    ()
}
