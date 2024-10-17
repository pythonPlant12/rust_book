fn main() {
    let number = 5;
    // We cannot do if number { ... } as in Python which is automatically convert non-Boolean values
    // to a Boolean values as Rust is statically typed language
    if number < 5 {
        println!("The number is less than 5");
    } else if number > 5 {
        println!("The number is greater than to 5");
    } else {
        println!("The number is 5");
    }
    // We can also make some complex conditions
    if number % 4 == 0 {
        println!("The number is divisible by 4");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3");
    } else if number % 2 == 0 {
        println!("The number is divisible by 2");
    } else {
        println!("The number is not divisible by 4, 3, or 2");
    }
    // Because if is an expression, we can use it on the right side of a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}
