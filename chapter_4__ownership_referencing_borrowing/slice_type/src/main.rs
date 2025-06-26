fn main() {
    let mut s = String::from("hello world");
    let index = first_word(&s);
    println!("First word {}", index);
    s.clear();
    // The problem here that index will be 5 even we cleared the s as we called first_word function before
    // clearing s string
    // println!("Len of space or string: {}", index);
    // let my_array: [&str; 2] = ["hi", "hello"];
    let mut hello: String = String::from("String type variable hello");
    let hi: String = String::from("String type variable hi");

    {
        let my_array1: [&String; 2] = [&hello, &hi];
        println!("my_array1: {:?}", &my_array1);
    } // References go out of scope here

    hello.push_str(" pushed more"); // Now we can mutate
    println!("hello after mutation: {}", hello);

    let mut s = String::from("hello world");
    let first_word = first_word(&s);
    // s.clear();
    println!("the first word is: {first_word}");


    // To avoid this sync problem, we should use strign slices
    // let s1 = String::from("hello world");
    // let hello = &s1[0..5]; // If you start with index 0, you can omit the 0 and do: &s[..5]
    // let world = &s1[6..11]; // If we use an index that is not valid, the programm will panic
    // If you want it to be till the end of the string slice, yo can do &s[6..] or let len = s1.len(); &s[6..len]
    // Or you can also use the full reference of the string doing &s[..]

    // Having all this in mind, let write first_word2 function and rewrite it using string slices
    // let s2 = String::from("hello world");
    // let index = first_word2(&s2);
    // println!("First word before space: {}", index);

    // let s = "Hello, world!"; // Here the type of s is &str and is immutable as it is a string slice and not String
    // in this case if we'd call the fn first_word we would type the parameter as &str instead of &String
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    println!("b{:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
//
// fn first_word2(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }
