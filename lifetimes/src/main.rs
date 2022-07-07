fn main() {
    // Validating References with Lifetimes

    // Lifetimes are another kind of generics that we've already been using.
    // Rather than ensuring that a type has the behavior we want, lifetimes
    // ensures that references are valid as long as we need them to be.

    // Every reference in Rust has a *lifetime*, which is the scope for which
    // that reference is valid. Most of the time, lifetimes are implicit and
    // inferred, just like most of the time, types are inferred.

    // We only must annotate types when multiple types are possible.
    // In a similar way, we must annotate lifetimes when the lifetimes of
    // references could be related in a few different ways.

    // Rust requires us to annotate the relationships using generic lifetime
    // parameters to ensure the actual references used at runtime will definitely
    // be valid.

    // ### Preventing Dangling References with Lifetimes
    // The main aim of lifetimes is to prevent *dangling references*, which cause
    // a program to reference data other than the data it's intended to reference.
    // {
    //     let r;  // If we try to use this variable, we get a compile-time error.
    //
    //     {
    //         let x = 5;  // `x` doesn't live long enough.
    //         r = &x;
    //     }
    //
    //     println!("r: {}", r);
    // }

    // ### The Borrow Checker
    // The Rust compiler has a *borrow checker* that compares scopes to determine
    // whether all borrows are valid.

    // ### Generic Lifetimes in Functions
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("{}", result);

    // ### Lifetime Annotation Syntax
    // Lifetime annotations don't change how long any of the references live!
    // Rather, they describe the relationships of the lifetimes of multiple references
    // to each other without affecting the lifetimes.
    // Functions can accept references with any lifetime by specifying a generic lifetime
    // parameter.

    // Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters
    // must start with an apostrophe (`'`) and are usually all lowercase and very short.
    // Most people use the name `'a` for the first lifetime annotation. We place lifetime
    // parameter annotations after the `&` of a reference, using a space to separate the
    // annotation from the reference's type.
    // Examples:
    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    // One lifetime annotation by itself doesn't have much meaning, because the annotations
    // are meant to tell Rust how generic lifetime parameters of multiple references relate
    // to each other.

    // let string1 = String::from("long string is long");
    // 
    // {
    //     let string2 = String::from("xyz");
    //     let result = longest(string1.as_str(), string2.as_str());
    //     println!("Te longest string is {}", result);
    // }

    // let string1 = String::from("long string is long");
    // let result;
    //
    // {
    //     let string2 = String::from("xyz");
    //     // Lifetime: same as the smaller of the lifetimes of the references passed in.
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    // ### Lifetime Annotations in Struct Definitions
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);

    // ### The Static Lifetime
    // One special lifetime we need to discuss is `'static`, which denotes that the affected
    // reference *can* live for the entire duration of the program. All string literals have
    // the `'static` lifetime, which we can annotate as follows:
    let s: &'static str = "I have a static lifetime";
    // The text of this string is stored directly in the program's binary, which is always
    // available. Therefore, the lifetime of all string literals is `'static`.

    // We might see suggestions to use the `'static` lifetime in error messages.
    // But before specifying `'static` as the lifetime for a reference, think about whether the
    // reference we have actually lives the entire lifetime of our program or not, and whether
    // we want it to.
    // Most of the time, the solution is fixing those problems, not specifying the `'static`
    // lifetime.
}

// Rust cannot tell whether the reference being returned refers to `x` or `y`.
// To fix this error, we'll add generic lifetime parameters that define the relationship
// between the references so the borrow checker can perform its analysis.
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// ### Lifetime Annotations in Function Signatures
// As with generic type parameters, we need to declare generic lifetime parameters
// inside angle brackets between the function name and the parameter list.
// We want the signature to express the following constraint: the returned
// reference will be valid as long as both the parameters are valid.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Only that some scope can be substituted for `'a` that will satisfy this signature.
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// The function signature now tells Rust that for some lifetime `'a`, the function
// takes two parameters, both of which are string slices that live at least as long as
// lifetime `'a`. The function signature also tells Rust that the string slice returned
// from the function will live at least as long as `'a`.
// In practice, it means that the lifetime of the reference returned by the `longest`
// function is the same as the smaller of the lifetimes of the references passed in.

// NOTE: We're specifying that the borrow checker should reject any values that don't
// adhere to these constraints.

// ### Thinking in Terms of Lifetimes
// The way in which we need to specify lifetime parameters depends on what our function
// is doing.
// For example, if we changed the implementation of the `largest` function to always return
// the first parameter rather than the longest string slice, we wouldn't need to specify
// a lifetime on the `y` parameter.
// fn first<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }

// ### Lifetime Annotations in Struct Definitions
// So far, the structs we've define all hold owned types. We can define structs to hold
// references, but in this case we would need to add a lifetime annotation on every
// reference in the struct's definition.
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}
// This annotation means an instance of `ImportantExcerpt` cannot outlive the reference
// it holds in its `part` field.

// ### Lifetime Elision
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
// Some patterns were programmed into the compiler's code so the borrow checker
// could infer the lifetimes in these situations and wouldn't need explicit annotations.

// The patterns programmed into Rust's analysis of references are called the *lifetime
// elision rules*. These aren't rules for programmers to follow; they're a set of particular
// cases that the compiler will consider, and if our code fits these cases, we don't need to
// write the lifetimes explicitly.

// Lifetimes on function or method parameters are called *input lifetimes*, and lifetimes on
// return values are called *output lifetimes*.

// The compiler uses three rules to figure out the lifetimes of the references when there aren't
// explicit annotations. If the compiler gets to the end of the three rules and there are still
// references for which it can't figure out lifetimes, the compiler will stop with an error.
// These rules apply to `fn` definitions as well as `impl` blocks.

// The first rule is that the compiler assigns a lifetime parameter to each parameter that's a
// reference.

// The second rule is that, if there is exactly one input lifetime parameter, that lifetime is
// assigned to all output lifetime parameters.

// The third rule is that, if there are multiple input lifetime parameters, but one of them is
// `&self` or `&mut self` because this is a method, the lifetime of `self` is assigned to all
// output lifetime parameters.

// ### Lifetime Annotations in Method Definitions
// Lifetime names for struct fields always need to be declared after the `impl` keyword and then
// used after the struct's name, because those lifetimes are part of the struct's type.

// In method signatures inside the `impl` block, references might be tied to the lifetime of
// references in the struct's fields, or they might be independent.
// In addition, the lifetime elision rules often make it so that lifetime annotations aren't
// necessary in method signatures.
impl<'a> ImportantExcerpt<'a> {
    // No annotation needed because of the first elision rule.
    fn level(&self) -> i32 {
        3
    }

    // The third lifetime elision rule applies!
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention pleaase: {}", announcement);
        self.part
    }
}

// ## Generic Type Parameters, Trait Bounds, and Lifetimes Together
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(  // Lifetimes are a type of generic.
                                         x: &'a str,
                                         y: &'a str,
                                         ann: T,  // Any type that implements the `Display` trait as specified by the `where` clause.
) -> &'a str
    where
        T: Display,
{
    println!("Announcement! {}", ann);  // This is why the `Display` trait bound is necessary.
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// NOTE: All of these analyses happen at compile time, which doesn't affect runtime performance.