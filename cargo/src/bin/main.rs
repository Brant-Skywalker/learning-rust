fn main() {
    // Managing Growing Projects with Packages, Crates, and Modules

    // The programs we've written so far have been in one module in one file.
    // As a project grows, we can organize code by splitting it into multiple modules
    // and then multiple files.
    // A packages can contain multiple binary crates and optionally one library crate.
    // As a package grows, we can extract parts into separate crates that become external
    // dependencies.
    // For very large projects of a set of interrelated packages that evolve together,
    // Cargo provodes workspaces.

    // We can create scopes and change which names are in or out of scope.
    // We can't have two items with the same name in the same scope.

    // The *module system* include:
    // * Packages: A Cargo feature that lets us build, test, and share crates
    // * Crates: A tree of modules that produces a library or executable
    // * Modules and use: Let us control the organization, scope, and privacy of paths
    // * Paths: A way of naming an item, such as a struct, function, or module

    // ### Packages and Crates
    // The first parts of the module system we'll cover are packages and crates.
    // A *package* is one or more crates that provide a set of functionality. A package
    // contains a *Cargo.toml* file that describes how to build those crates.

    // A *crate* can be a binary crate or library crate. *Binary crates* are programs we can
    // compile to an executable that we can run. They must have a function called `main`.

    // *Library crates* don't have a `main` function, and they don't compile to an executable.
    // They define functionality intended to be shared with multiple projects.

    // The *crate root* is a source file that Rust compiler starts from and makes up the root module
    // of our crate.

    // A package can contain at most one library crate. It can contain as many binary crates as we'd
    // like, but it must contain at least one crate (either library or binary).

    // When we enter `cargo new my-project`, Cargo created a *Cargo.toml* file, giving us a package.
    // Looking at the contents of *Cargo.toml*, there's no mention of `src/main.rs` because Cargo
    // follows a convention that `src/main.rs` is the crate root of a binary crate with the same
    // name as the package.
    // Likewise, Cargo knows that if the package contains `src/lib.rs`, the package contains a
    // library crate with the same name as the package, and *src/lib.rs* is its crate root.
    // Cargo passes the crate root files to `rustc` to build the library or binary.

    // If a package contains `src/main.rs` and `src/lib.rs`, it has two crates: a binary and a
    // library, both with the same name as the package. A package can have multiple binary crates
    // by placing files in the `src/bin` directory: each file will be a separate binary crate.
}
