fn main() {
    let printed_value = printing_value();
    println!("{}", printing_value());
}

// You can define your function anywhere in the file
fn printing_value() -> u8 {
    // This is returning a value of type u8 as we don't add a semicolon at the end
    8
}
