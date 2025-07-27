use List::{Cons, Nil};

fn main() {
    // Box<T>. Is kind of "box" which stores a smart pointer.
    // The smart pointer can contain more information that only allocation only heap, like metadata
    // or more.
    // One of them is Box<T>, this allows us to be able to work with recursivness.
    // For example. If we'd do it without Smart Pointer Box<T>, this would fail as the compiler
    // won't know at compile time, how many memory to allocate on stack for this variable for every
    // List created. (It could be infinite pointers).
    // To avoid that, we create Box::new, this will create a pointer on stack for let list,
    // however, all other pointers will be stacked on the heap, so it's memory can vary and avoid
    // infinite pointers created and compile time. Instead this pointers will be managed on the
    // heap, by the price of overhead.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Compiled");
    //
    // Let's take a look to the Deref. Dereferencing allows us to access the data of the pointer
    // rather than the pointer itself.
    let x = 5;

    let y = &x;
    assert_eq!(5, x);
    // This will not work as we compare the pointer to the integer itself.
    // assert_eq!(5, y);
    //
    // However, if you do it with Deref, you'll access directly to the value rather than the
    // pointer;
    assert_eq!(5, *y);

    // =====================================
    // Memory allocations checks
    // =====================================
    //
    let x = 5;
    let y = &x;
    let z = Box::new(10);

    println!("x value: {}", x); // Prints: 5
    println!("y pointer: {:p}", y); // Prints: 0x7fff... (stack address)
    println!("z pointer: {:p}", z.as_ref()); // Prints: 0x... (heap address)

    // Check memory addresses:

    let stack_var = 42;
    let heap_var = Box::new(42);

    println!("Stack address: {:p}", &stack_var); // Higher address (0x7fff...)
    println!("Heap address: {:p}", heap_var.as_ref()); // Lower address (0x...)

    // General patterns:

    // - Stack addresses: Usually start with 0x7fff... (high memory)
    // - Heap addresses: Usually lower values like 0x600... or 0x1...

    // Debug size:

    use std::mem;
    println!("Size of i32: {}", mem::size_of::<i32>()); // 4 bytes
    println!("Size of &i32: {}", mem::size_of::<&i32>()); // 8 bytes (pointer)
    println!("Size of Box<i32>: {}", mem::size_of::<Box<i32>>()); // 8 bytes (pointer)
    //
    //
    // ==============================================
    // Using Box<T> Like a Reference
    // ==============================================
    // Here we need to use Deref to follow the pointer to it's actual value too, as Box<T>
    // implements Deref trait
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Let's build a smart pointer similar to the Box<T> type provided by the standard library to
    // experience how smart pointers behave differently from references by default. Then we'll look
    // at how to add the ability to use the dereference operator.
    struct MyBox<T>(T); // The MyBox type is a tuple struct

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    let x = 5;
    let y = MyBox(x);
    assert_eq!(5, x);
    // assert_eq!(5, *y); // It cannot be dereferenced as we didn't implement the Deref trait
    //
    // =========================================
    // Implementing the Deref trait
    // =========================================
    //
    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    // This is more less how deref is implemented in Deref.
    // When we call *y, behind the scenes ito does: *(y.deref())
    //
    // Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type. For example, deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str. Deref coercion is a convenience Rust performs on arguments to functions and methods, and works only on types that implement the Deref trait. It happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition. A sequence of calls to the deref method converts the type we provided into the type the parameter needs.[27;5;106~[27;5;106~Deref coercion was added to Rust so that programmers writing function and method calls don’t need to add as many explicit references and dereferences with & and *. The deref coercion feature also lets us write more code that can work for either references or smart pointers.[27;5;106~[27;5;106~To see deref coercion in action, let’s use the MyBox<T> type we defined in Listing 15-8 as well as the implementation of Deref that we added in Listing 15-10. Listing 15-11 shows the definition of a function that has a string slice parameter.
    fn hello(name: &str) {
        println!("hello {name}");
    }
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // Here we’re calling the hello function with the argument &m, which is a reference to a MyBox<String> value. Because we implemented the Deref trait on MyBox<T> in Listing 15-10, Rust can turn &MyBox<String> into &String by calling deref. The standard library provides an implementation of Deref on String that returns a string slice, and this is in the API documentation for Deref. Rust calls deref again to turn the &String into &str, which matches the hello function’s definition.[27;5;106~[27;5;106~If Rust didn’t implement deref coercion, we would have to write the code in Listing 15-13 instead of the code in Listing 15-12 to call hello with a value of type &MyBox<String>.
    // If we wouldn't implement the Deref trait. We'd need to do something like:
    let m_no_deref = MyBox::new(String::from("Rust"));
    hello(&(*&m_no_deref)[..]);
    }
    // The (*m) dereferences the MyBox<String> into a String. Then the & and [..] take a string slice of the String that is equal to the whole string to match the signature of hello. This code without deref coercions is harder to read, write, and understand with all of these symbols involved. Deref coercion allows Rust to handle these conversions for us automatically.[27;5;106~[27;5;106~When the Deref trait is defined for the types involved, Rust will analyze the types and use Deref::deref as many times as necessary to get a reference to match the parameter’s type. The number of times that Deref::deref needs to be inserted is resolved at compile time, so there is no runtime penalty for taking advantage of deref coercion!
    //
    // =========================
    // How Deref Coercion Interacts with Mutability
    // =========================
    // Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.
    // Rust does deref coercion when it finds types and trait implementations in three cases:

    // From &T to &U when T: Deref<Target=U>
    // From &mut T to &mut U when T: DerefMut<Target=U>
    // From &mut T to &U when T: Deref<Target=U>
    // The first two cases are the same except that the second implements mutability. The first case states that if you have a &T, and T implements Deref to some type U, you can get a &U transparently. The second case states that the same deref coercion happens for mutable references.
    //
    // The third case is trickier: Rust will also coerce a mutable reference to an immutable one. But the reverse is not possible: immutable references will never coerce to mutable references. Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data (otherwise, the program wouldn’t compile). Converting one mutable reference to one immutable reference will never break the borrowing rules. Converting an immutable reference to a mutable reference would require that the initial immutable reference is the only immutable reference to that data, but the borrowing rules don’t guarantee that. Therefore, Rust can’t make the assumption that converting an immutable reference to a mutable reference is possible.
    //

enum List {
    Cons(i32, Box<List>),
    Nil,
}
