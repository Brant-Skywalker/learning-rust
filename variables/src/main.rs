fn main() {
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;  // Cannot assign twice to immutable variable `x`!
    // println!("The value of x is: {}", x);

    /// Make variables **mutable** by adding `mut` in front of the variable name.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    /// **Constants**
    /// We are not allowed to use `mut` with constants. Constants can be declared using the `const`
    /// keyword instead of the `let` keyword, and the type of the value *must* be annotated.
    /// NOTE: Constants may be set only to a constant expression, not the result of a value
    /// that could only be computed at runtime.
    /// Naming convention: use all uppercase with underscores between words.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    /// Constants are valid for the entire time a program runs, **within the scope they were
    /// declared in**.

    /// **Shadowing**
    /// We can shadow a variable by using the same variable's name and repeating the use of the
    /// `let` keyword.
    let x = 5;      // Shadowed!
    let x = x + 1;  // Shadowed!
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    /// We are effectively creating a new variable. When we use the `let` keyword again,
    /// we can change the type of the value but reuse the same name.
    let spaces = "   ";   // String type.
    let spaces = spaces.len();  // Number type.

    /// We are not allowed to mutate a variable's type.
    // let mut spaces = "   ";
    // spaces = spaces.len();  // Compiler error!


}
