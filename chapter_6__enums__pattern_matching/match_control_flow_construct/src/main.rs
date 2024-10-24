fn main() {
    // Rust match control flow allows you to compare a value agains a series of patterns and then execute code based on which pattern matches.
    // The petterns can be made up of literal values, variable names, wildcards and many others. We'll cover more cases in Chapter 18
    // E.g. Think of match as coin sorting machine.

    // As we took example of coins, let's reproduce this example in code
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter
    }

    fn value_in_cents(coin: Coin) -> u8 {
        // Match is similar to if statement, however, if only evaluates boolean values, match can evaluate any type of value.
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25
        }
    }
    // Next are match arms. An arm has two parts, a pattern and some code:
    // The first are has a oattern that is the value Coin::Penny
    // The => operator that separates the pattern and the code to run. Each arms are separated by the comma ','
    // The code associated with each arm is an expression, and the resultant value of the expression
    // in the matching amr is the value that gets returned for the entire match expression

    // If match code is short we don't tipically use {}
    // If you want to execute multiple lines of code, you should use {}, and in this code the comma ',' is not required
    // In the next code, it will print Lucky penny if matches and return value of 1
    fn value_in_cents1(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25
        }
    }
    value_in_cents1(Coin::Penny);

    // Another useful feature of match arms is that they can bind to the parts of the values that match the pattern.
    // This is how we can extract values out of enum variants
    #[derive(Debug)]
    enum UsState1 {
        Alabama,
        Alaska
    }

    enum Coin1 {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState1)
    }
    fn value_in_cents2(coin1: Coin1) -> u8 {
        // When a Coin1:Quarter matches, the state variable will bind tothe value of that quarter's state.
        // Then we can use that binding in the code for that arm
        match coin1 {
            Coin1::Penny => 1,
            Coin1::Nickel => 5,
            Coin1::Dime => 10,
            Coin1::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
    value_in_cents2(Coin1::Quarter(UsState1::Alabama));


    // Now, instead of comparing coins, we'll compare with match Option<T> values
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let none = plus_one(None);

    // It is very IMPORTANT that the match should cover all the possibilities. Otherwise the programm
    // will not compile
    // Match patterns are "exhaustive", we must exhaust every last possibility in order for the code to be valid


    // There is a possibility to cover some of them and one for the rest
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // The last arm covers very other possible value, the pattern is the variable we've chosen to name other1
        // This catch-all pattern meets the requirements that match must be exhaustive as it covers the rest of the possible values
        // other1 => move_player(other1), // For the last case we can use different variable
        // But if we don't use the variable other1, we can use _ to ignore the value of using it
        // _ => reroll() // Instead of calling empty function as we don't want to do anything
        // We can you () a unit type with empty Tuple
        _ => ()
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}


    // We'll cover more about match in Chapter 18
}
