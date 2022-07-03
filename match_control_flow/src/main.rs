fn main() {
    // The power of `match` comes from the expressiveness of the patterns and the
    // fact that the compiler confirms that all possible cases are handled.
    // enum Coin {
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter,
    // }
    //
    // fn value_in_cents(coin: Coin) -> u8 {
    //     match coin {  // First we list the `match` keyword followed by an expression.
    //         Coin::Penny => {
    //             println!("Lucky penny!");
    //             1
    //         }  // `match` arms.
    //         Coin::Nickel => 5,
    //         Coin::Dime => 10,
    //         Coin::Quarter => 25,
    //     }
    //     // An arm has two parts: a pattern and some code.
    //     // When the `match` expression executes, it compares the resulting value against
    //     // the pattern of each arm, in order. If a pattern matches the value, the code
    //     // associated with that pattern is executed.
    //     // If that pattern does not match the value, execution continues to the next arm.
    //
    //     // We don't typically use brackets if the match arm code is short.
    // }
    //
    // println!("{}", value_in_cents(Coin::Penny));

    // Patterns that Bind to Values
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    // Matching with `Option<T>`
    // We cal also handle `Option<T>` using `match` as we did with the `Coin`
    // enum! We'll compare the variants of `Option<T>`.
    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         None => None,
    //         Some(i) => Some(i + 1)
    //     }
    // }
    //
    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    // Matches Are Exhaustive
    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {  // Won't compile.
    //         Some(i) => Some(i + 1)
    //     }
    // }
    // Rust knows that we didn't cover every possible case and even knows which
    // pattern we forgot! Matches in Rust are *exhaustive*: we must exhaust every
    // last possibility in order for the code to be valid.

    // Catch-all Patterns and the _ Placeholder
    // Using enums, we can also take special actions for a few particular values,
    // but for all other values take one default action.
    // let dice_roll = 9;
    // match dice_roll {
    //     3 => add_fancy_hat(),
    //     7 => remove_fancy_hat(),
    //     other => move_player(other)  // Must be the last arm.
    // }
    //
    // fn add_fancy_hat() {}
    // fn remove_fancy_hat() {}
    // fn move_player(num_spaces: u8) {}

    // Rust also has a pattern we can use when we don't want to use the value in
    // the catch-all pattern: `_`, which is a special pattern that matches any value
    // and does not bind to that value.
    // let dice_roll = 9;
    // match dice_roll {
    //     3 => add_fancy_hat(),
    //     7 => remove_fancy_hat(),
    //     _ => reroll()
    // }
    //
    // fn add_fancy_hat() {}
    // fn remove_fancy_hat() {}
    // fn reroll() {}

    // We can express "nothing else happens" by using the unit value.
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => ()  // We aren't going to use any other value that does not match
        // a pattern in an earlier arm, and we don't want to run any code in this case.
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}
