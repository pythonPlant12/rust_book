use core::borrow;
use std::{thread, time::Duration};

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?}, gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // Let's define a closure in a variable (as you can see it looks more like a function syntax)
    let expensive_closure = |num: i32| -> i32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    // Let's see how closures are similar to functions in syntax
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // These last two will work, however, by themselves, they will need a type annotation similar
    // to Vec::new()
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;
    //
    // For closures, the compiler will infer the type for each of their parameters and for their
    // return value, for example, this closure is able to be used with different types
    let example_closure = |x| x;
    let s = example_closure(String::from("Hello"));
    // The closure can receive a param of any type, however, if it is used once, it will infer this
    // type and following calling of this closure should be of the same type
    // let n = example_closure(5);
    //
    // Closures, same as functions can capture values from their environment in three ways:
    // 1. Borrow immutably
    // 2. Borrow mutably
    // 3. Taking ownership
    //
    //
    // They way a closure captures and handles values from the environment affects which traits the
    // closure implements, and traits are how functions and structs can specify what kinds of
    // closures they can use. Closures will automatically implement one, two, or all three of these
    // Fn traits, in an additive fashion, depending on how the closure's body handles the values.
    // 1. FnOnce - applies to closures that can be called once. All closures implement at least
    //    this trait because all closures can bed called. A closure that moves captured values out
    //    of it's body will only implement FnOnce and none of the other Fn traits, because it can
    //    only be called once.
    // 2. FnMut - applies to closures that don't move captured values out of their body, but that
    //    might mutate the captured values. These closures can be called more than once.
    // 3. Fn - applies to closures that don't move captured values out of their body and that don't
    //    mutate captured values, as well as closures that capture nothing from their environment.
    //    These closures can be called more than once without mutating their environment, which is
    //    important in cases such as calling a closure multiple times concurrently.
    //
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");

    only_borrows();
    println!("After calling closure: {list:?}");
    //
    // Next we change the closure body so that it adds an element to the list vector. The closure
    // now captures a mutable reference
    let mut list1 = vec![1, 2, 3];
    println!("Before defining closure: {list1:?}");

    let mut borrows_mutably = || {
        println!("Pushing new value to a list1 from the closure borrows_mutably");
        list1.push(7);
    };

    // println!("After defining a closure: {list1:?}"); // This is not allowed as we cannot use
    // // multiple mutable borrow
    borrows_mutably();
    println!("After calling closure: {list1:?}");

    // If you want the closure to force to take ownership, you should use "move" keyword.
    // This is useful when you have multiple threads and you want to pass the data so that's owned
    // by the new thread
    //
    let list2 = vec![1, 2, 3];
    println!("Before defining closure list2: {list2:?}");

    thread::spawn(move || println!("From thread list2: {list2:?}"))
        .join()
        .unwrap();
    // println!("Before defining closure list2: {list2:?}"); // Here it will not be able to call as
    // the ownership is moved to a new thread
    //
    //
    // unwrap_or_else() impelements FnOnce, so the closure will be called as most one time.
    // If you don't want to pass any param, you can also do unwrap_or_else(Vec::new);
    //
    // Let's see how it changes from FnOnce to FnMut
    let mut rectangles_list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height:5 },
        Rectangle { width: 7, height: 12 },
    ];

    // Closure that implements FnMut
    rectangles_list.sort_by_key(|r| r.width);
    println!("{rectangles_list:#?}");

    // Closure that implements FnOnce
    let mut sort_operations: Vec<String> = vec![];
    let value = String::from("closure called");

    let mut counted_operations: u32 = 0;

    rectangles_list.sort_by_key(|r| {
        counted_operations += 1;
        println!("{counted_operations}");

        // sort_operations.push(value); // If we add this here it will not work, however, if we
        // implement clone() yes.
        // This happens because this closure is called multiple times, and if we try to push value
        // from the outer environment, we'll transfer the ownership of first calling closure, so on
        // second call the value will not have the ownership
        // 
        // However, we can do the following:
        //
        r.width
    });
    println!("rectangles_list: {rectangles_list:#?}");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
