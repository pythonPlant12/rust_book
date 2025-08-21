use std::vec;

fn main() {
    // The term `macro` referes to a family of features in Rust:
    // `declarative` macros with `macro_rules!` and three kinds of `procedural` macros
    // - #[derive] macros that specify code added with the `derive` attribute used on sructs and enums
    // - attribute-like macros that look like attributes but can be used on any item
    // - function-like macros that look like function calls but operate on the tokens specified as their argument

    // So why we need macros?
    // Macros allow us to write code that writes other code, which can reduce boilerplate
    // It is called metaprogramming
    // However, macros have some additional powers that functions do not have
    // For example prinltn!("hello"), and println!("helo {}", name) (one doesn't take arguments, the other does)

    // The downside to implementing macro instead of a function is that macro definitions are more
    // complex. Because you are writing code that writes code.

    // Another important difference between macros and functions is that you must define macros or
    // bring them into scope `before` you call them in a file, as opposed to functions you can define anywhere
    // and call anywhere.

    // To define a macro, you use the `macro_rules!` consctruct.
    // Let's see how the macro vec! is defined

    let v: Vec<u32> = vec1![1, 2, 3, 4, 5];
}

// Thanks to the macro, we can use all the arguments, with function it would be impossible as we don't know how many arguments we will have
// The #[macro_export] annotation indicates that this macro should be made available whenever the crate in which
// the macro is defined is brought into scope. Without this annotation, the macro can’t be brought into scope.

#[macro_export]
macro_rules! vec1 {
    // Macro syntax is similar to match expressions, if the pattern matches, the code in the body is executed
    // First use set of parenthesis to encompass whe whole pattern... Then a $ to declare a variable in the macro system
    // That will contain the Rust code matching the pattern. This $ means that is macro variable, not a Rust variable
    // When we call the [1,2,3] the code matches 3 times.
    ($($x:expr),*) => {
        { 
            let mut temp_vec = Vec::new();
            $(             
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
    // This is what is translated to:
    // let mut temp_vec = Vec::new();
    // temp_vec.push(1);
    // temp_vec.push(2);
    // temp_vec.push(3);
    // temp_vec
}
// Note: The actual definition of the vec! macro in the standard library includes code to pre-allocate the correct amount of memory up front.
// That code is an optimization that we don’t include here, to make the example simpler.
