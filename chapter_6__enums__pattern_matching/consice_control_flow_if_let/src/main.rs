fn main() {
    // If let syntax is useful if you want to handle values that match only one or some of the patterns.
    let config_max: Option<u8> = Some(12u8);
    // match config_max {
    //     Some(max) => println!("Max value is: {max}"),
    //     _ => ()
    // }
    match config_max {
        Some(value) => println!("Yes {value}"),
        _ => println!("No")
    };


    // With if let syntax, you can write the above code in a more concise way and not cover all the cases.
    // We should do this in order to bind the value from the if statement
    if let Some(max) = config_max {
        println!("Max value is: {max}. Max is binded value from the if statement");
    }

    // In case we don't need to bind the value we can do something like this
    if config_max == Some(12u8) {
        println!("Without binding the value from the if statement")
    }

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState)
    }
    // Here we compare the coin to an specific variant of the enum, and then the value state bind to the value of that quarter's state on the right
    if let Coin::Quarter(state) = Coin::Quarter(UsState::Alabama) {
        println!("State quarter from {:?}!", state);
    } else {
        println!("Not a state quarter");
    }
    let quarter_coin = Coin::Quarter(UsState::Alaska);

    if let Coin::Quarter(state) = quarter_coin {
        println!("State quarter from {:?}!", state);
    } else {
        println!("Not a state quarter");
    }

    let one = 1;
    let two = 2;
    if one > two || one == two {
        println!("One is greater or equal than two");
    } else {
        println!("One is not greater than two");
    }
}
