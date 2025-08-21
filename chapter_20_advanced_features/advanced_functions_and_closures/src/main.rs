fn main() {
    // Beside of passing closures in functions, we can also pass already defined functions.
    // The fn type is called a `function pointer`. Passing functions with function pointers will allow you to use functions as arguments to other function.

    // The syntax for specifying that a parameter is a function pointer is similar to that of closures.
    do_twice(add_one, 5);
    println!("do_twice(add_one, 5) = {}", do_twice(add_one, 5));
    // Unlike closures, `fn` is a type rather than a trait, so we specify fn as the parameter type directly rather than declaring a generic type parameter with one of the `Fn` traits.


    // Let's take the example of how it would look with closure for comparison:
    let closure = |x: i32| x + 1;
    let result = do_twice(closure, 5); // Even we can use the same function with param type is fn(i32),
    // The correct way would be to do:
    // Accepts any closure or function that implements Fn(i32) -> i32
    // fn do_twice<F>(f: F, arg: i32) -> i32
    // where
    //     F: Fn(i32) -> i32,
    // {
    //     f(arg) + f(arg)
    // }
    println!("do_twice(closure, 5) = {}", result);

    // Function pointers implement all three of the closure traits (`Fn`, `FnMut`, and `FnOnce`),
    // so you can use a function pointer in any situation where you can use a closure that implements one of those traits.
    // One example of where you would want to only accept `fn` and not closures is when 
    // ihnterfacing with external code that doesn't have closures: C functions can accept functions as arguments, but C doesn't have closures.

    // Let's see another example of fn pointer and closure.
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|x| x.to_string()).collect();

    println!("list_of_strings = {:?}", list_of_strings);

    let list_of_strings1: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("list_of_strings1 = {:?}", list_of_strings1);

    // We can also use `Enum` because it also becomes an initializer function.
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (1u32..=20).map(Status::Value).collect();
    println!("list_of_statuses = {:?}", list_of_statuses);


    // ================================
    // Returning closures
    // ================================

    // Closures are represented by traits, which menas you can't return closures directly. In most cases
    // where you might want to return a trait, you can instead use the concrete type that implements the trait
    // as the return value of the function. However, you can't usually do that with closures because the don't have a concrete type that is returnable. 
    // You're not allowed to use the function pointer `fn` as a return type if the closure captures any values from its scope.

    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    // The error message tells us that wheenever we return an `impl Trait` Rust creates a unique `opaque type`
    // a type where we cannot see into the details of what Rust constructs for us. 
    // For this case, we need to use Box<dyn>

}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// Instead, you will normally use the `impl Trait`. You can return any function type, using `Fn`, `FnOnce` or `FnMut`.
// fn returns_closure() -> impl Fn(i32) -> i32 {
//     |x| x + 1
// }
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// However, each closure is also its own distinct type. If you need to work with multiple functions that have the same signature but different
// implementations, you'll need to use a trait object for them.
// fn returns_initialized_closure(init: i32) -> impl Fn(i32) -> i32 {
//     move |x| x + init
// }
fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}