mod front_of_house; // First we extract the `front_of_house` module to its own file.
// We only need to load the contents of a file using a `mod` declaration once somewhere
// in our module tree.

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}