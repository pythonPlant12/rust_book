fn main() {
    // String literals like "hello" are immutable, and their values are hardcoded into the program.
    // In order to have a mutable string, we need to use the String type.
    let mut s: String = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    println!("-----------------------------");

    // Here even if i change the value of x afterwards, the value of y will not change.
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    // L
    let s1 = String::from("hello");

    // If we only do s2 = s1, then s1 will be invalidated and we will not be able to use it.
    // This only happens with data stored in the heap,
    // not with data stored in the stack as the memory is allocated on compile time and cannot change.
    // let s2 = s1;

    // Cloning is an expensive operation as you copy all the data in the heap to .
    let s2 = s1.clone();
    println!("s2: {s2}");
    println!("s1: {s1}");

    println!("-----------------------------");

    // OWNERSHIP AND FUNCTIONS
    let s: String = String::from("hello");
    takes_ownership(s);
    // In this case, s is no longer valid as it was moved to the function. And was dropped when the function ended.
    // So this prinltn!() is not possible.
    // println!("{s}");
    let x: i32 = 5;
    makes_copy(x);
    // In this case the x value is still available is is stack data, and once it is passed to the function,
    // it is copied and not moved.
    println!("{x}");

    println!("-----------------------------");
    // RETURNING VALUES AND SCOPE
    let s1: String = gives_ownership(); // gives_ownership moves its return value into s1
    let s2: String = String::from("hello"); // s2 comes into scope
    let s3: String = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3, so in this ase s2 should be invalid after calling this function
    println!("{s3}");

    // We can also return multiple values from a function by returning a tuple.
    let (s4, len): (String, usize) = calculate_length(s1);
    println!("The length of '{s4}' is {len}");
}


fn takes_ownership(s: String) { // some_string comes into scope
    println!("{s}");
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(x: i32) { // some_integer comes into scope
    println!("{x}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it
    let some_string: String = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length: usize = s.len(); // len() returns the length of a String
    (s, length)
}