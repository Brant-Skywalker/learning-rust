mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// We define a module by starting with the `mod` keyword and then specify the name of the module
// and place curly brackets around the body of the module.
// Inside modules, we can have other modules. Modules can also hold definitions for other items,
// such as structs, enums, constraints, traits, or functions.

// By using modules, we can group related definitions together and name why they're related.

// The entire module tree is rooted under the implicit module named `crate`.
