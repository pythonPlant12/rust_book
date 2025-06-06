fn main() {
    // Example of inmutable references and borrowing
    let s1: String = String::from("hello");
    let len: usize = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Example of mutable references and borrowing
    // You can have only ONE mutable reference at a time
    let mut s1 = String::from("hello");
    change_s1(&mut s1);
    println!("{}", s1);

    // Trying to use two &mut references at the same time
    // let mut s2: String = String::from("hello");
    // let r1: &mut String = &mut s2;
    // let r2: &mut String = &mut s2;
    // println!("{}, {}", r1, r2); // This line will cause an error

    // Trying to use two & references and one &mut reference at the same time
    // let mut s2: String = String::from("hello");
    // let r1: &String = &s2; // No problem here as is an inmutable reference
    // let r2: &String = &s2; // No problem here as is an inmutable reference
    // let r3: &mut String = &mut s2; // This line will cause an error as we have two inmutable references and one mutable reference at the same time
    // println!("{}, {}, {}", r1, r2, r3);

    // Take into consideration that if we use different scopes, we can have multiple references
    let mut s2: String = String::from("hello people");
    {
        let r1: &mut String = &mut s2;
        r1.push_str(", world");
        println!("{}", r1);
    }
    let r2: &mut String = &mut s2;
    // r1 value was changed in other scope, however, as it was a pointer or reference to an s2, the
    // value of s2 was changed as well, even in other scope, so the last pointer r2 will have the new value of s2
    println!("{}", r2);

    // Note that a reference's scope starts from where it is introduced and continues throught the last time that reference is used,
    // Based on this, the following code will compile
    let mut s3: String = String::from("string3");
    let r1: &String = &s3;
    let r2: &String = &s3;
    println!("{}, {}", r1, r2);
    // If this line would be before first println, we'd get error.
    let r3: &mut String = &mut s3;
    println!("{}", r3);

    // In languages with pointers, it is easy to erroneosly create a dangling pointer - a pointer
    // that references a location in memory that may have been given to someone else, by freeing some memory while preserving a pointer to that memory.
    // In Rust, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure the reference will always have access to the data.
    // The following code will not compile, as the reference to s is returned, but s is dropped at the end of the function.
    // let reference_to_nothing = dangle();


}
// Function of inmutable references and borrowing
fn calculate_length(string_pointer: &String) -> usize {
    string_pointer.len()
}

// Function of mutable references and borrowing
fn change_s1(string_pointer: &mut String) {
    string_pointer.push_str(", world");
}

// Dangle reference in a function
// fn dangle() -> &String {
//     let s: String = String::from("hello");
//     &s
// }