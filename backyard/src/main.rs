use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    // Modules Quick Reference
    // * Start from the crate root: When compiling a crate, the compiler first looks in the crate
    // root file (usually `src/lib.rs` for a library crate or `src/main.rs` for a binary crate).
    // * Declaring modules: In the crate root file, we can declare a new module named, say, "garden",
    // with `mod garden;`. The compiler will look for the code inside the module in these places:
    //      * Inline, directly following `mod garden`, within curly brackets instead of the semicolon
    //      * In the file `src/garden.rs`
    //      * In the file `src/garden/mod.rs`
    // * Declaring submodules: In any file other than the crate root that's being compiled as part
    // of the crate (for example, `src/garden.rs`), we can declare submodules (for example, `mod
    // vegetables;`). The compiler will look for the code inside submodules in these places within
    // a directory named for the parent module:
    //      * Inline, directly following `mod vegetable`, within curly brackets instead of the semicolon
    //      * In the file `src/garden/vegetable.rs`
    //      * In the file `src/garden/vegetable/mod.rs`
    // * Paths to code in modules: Once a module is being compiled as part of our code, we can refer
    // to code in that module from anywhere else in this crate by using the path
    // `crate::garden::vegetables::Asparagus` as long as the privacy rules allow.
    // * Private vs public: Code within a module is private from its parent modules by default.
    // To make a module public, declare it with `pub mod` instead of `mod`. To make items within a
    // public module public as well, use `pub` before their declarations.
    // * The `use` keyword: Within a scope, the `use` keyword creates shortcuts to items to reduce
    // repetition of long paths.

    // The crate root file, in this case `src/main.rs`, contains:
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);

    // Grouping Related Code in Modules
    // *Modules* let us organize code within a crate into groups for readability and easy reuse.
    // Modules also control the *privacy* of items, which is whether an item can be used by outside
    // code (*public*) or is an internal implementation detail and not available for outside use
    // (*private*).

}
