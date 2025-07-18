use std::fmt::Display;

pub mod structs_module;

fn main() {
    // Lifetimes are another kind of generic.
    // Rather than ensuring that a type has the behavior we want, lifetimes ensure that references
    // are valid as long as we need them to be.
    // Every reference in Rust has its lifetime
    // Normally, we don't need to specify a lifetime for references as compiler is doing it for us,
    // however, sometimes we'll need to do it manually.

    // Lets see some examples
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {r}")
    // This will not work, as we reference a variable that is going out of scope when we use it.
    // In this concrete example this called dangling references

    // Lets try another function
    let string1 = String::from("abcd");
    let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    let string_from_main_scope = String::from("main scope");
    // let result1;
    // {
    //     let string_from_secondary_scope = String::from("secondary scope");
        // result1 = longest(string_from_main_scope.as_str(), &string_from_secondary_scope.as_str());
    // }
    // println!("The longest string is {}", result1);
    
    // Now let's use the struct from struct_module
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = structs_module::ImportantExcerpt {
        part: first_sentence
    };


}
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }
// This will not compile, as we didn't add lifetime specifier. Why do we need it here?
// Two points to take into consideration, first are params the function needs. These are two str
// references, second, as we don't know what values are being passed when calling this function, we
// cannot say if x or y will be returned. And for how long will this references be valid at compile
// time.
//
// Lifetime annotation don't change how long any of the references live. Rather, they describe the
// relashionshiip of the lifetimes of multiple references to each other withouf affecting
// lifetimes.
// Examples of lifetimes for references:
// &i32
// &'a i32
// &'a mut i32
// One lifetime doesn't have much meaning because the annotations are meant to tell Rust how
// generic lifetime parameters of multiple references relate to each other.
//
// To use lifetime annotations in function signatures, we need to declare the generic lifetime
// parameter inside angle brackets between the function name and the parameter list, just as we
// didi with generic type parameters.
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() { x } else { y }
// }
// Withot lifetime specifiers this will not even compile
// fn longest_without_lifetime_annotations(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }
// Now, going to the main function, let's see how this can be useful callng this function
//
// If we'd return only one reference value, we don't even need to specify lifetime specifier for
// other parameter, here we don't specify the lifetime of y because it doesn't have any
// relashionship with x
fn longest_without_second_specifier<'a, 'b>(x: &'a str, y: &'b mut str) -> &'a str {
    x
}

// This will not compile because we are creating dangling reference because the value goes out of
// scope even with lifetime specifiers (remember, they don't modify the lifetime)
// This will not compile
// fn longest_with_dangling_reference<'a>(x: &'a str, y: &'a str) -> &'a str {
//     let new_string = String::from("hi");
//     new_string.as_str()
// }
//

fn first_word(word: &str) -> &str {
    let bytes = word.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &word[0..i];
        }
    }
    &word[..]
}

// There are three rules that compiler check for lifetimes, 1 for input lifetime (params) and two
// for output lifetime references
// 1. The compiler assigns input lifetimes to each parameter
// 2. The compiler If there is exactly one input lifetime parameter, that lifetime is assigned to
//    all output lifetime parameters
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self
//    because this is a method, the lifetime of self is assigned to all output lifetime parameters.
//    This third rule makes methods much nicer to read and write because fewer symbols are
//    necessary.
//
//    Let's pretend we're the compiler. We'll apply these rules to figure out the lifetimes of the
//    references in the signature of the first_word function. The signature starts without any
//    lifetimes associated with the references.
//
//    fn first_word(s: &str) -> &str {}
//    1. The compiler applies the first rule, which specifies that each parameter gets its own
//       lifetime.
//    fn first_word<'a>(s: &'a str) -> &str {}
//    2. The second rule appliues because there is exactly one input lifetime. The second rule
//       specifies that the lifetime of th eon einput parameter gest assigned to th eoutput
//       lifetime, so the signature is now this:
//    fn first_word<'a>(x: &'a str) -> &'a str {}
//    Now all the references in this function signature have lifetimes, and the compiler can
//    continue its analysis without needing the programmer to annotate the lifetimes in this
//    function signature.
//
//
// Let's see the example of longest function:
// fn longest(x: &str, y: &str) -> &str {}
// 1. The first rule is applied by the compiler, it assigns the input lifetimes:
// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
// 2. However, the second rule is not able to be applied as there are two input params for
//    references so the compiler doesn't know which will be returned, and it will not compile if we
//    don't specify the output lifetimes by ourselves.
//    
//
//    Let's try to rewrite the function longest, now taking into consideration everything from the
//    Chapter 10
//
// We can do it this way:
// fn longest<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str
// {
//     "hi"
// }
//
// Or this way:
fn longest<'a, 'b, T>(x: &'a str, y: &'b str, ann: T) -> &'a str 
where T: Display {
    "hi"
}
