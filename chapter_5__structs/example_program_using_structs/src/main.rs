fn main() {
   // To understand when we should use the structs, let's define some code
    let width1: i32 = 30;
    let height1: i32 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // Previous code is okay, but it's not clear what width1 and height1 are.
    // Let's create a rectangle with tuple
    let rect1: (i32, i32) = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    // Now is okay, but it's not clear what rect1 is. Let's create a rectangle with struct
    #[derive(Debug)]
    struct Rectangle {
        width: i32,
        height: i32,
    }
    let rect2 = Rectangle { width: 30, height: 50 };
    // Here we will have the error if defined function is outside the scope as we provide the type of the struct
    // Which is in different scope than the definition of the struct
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect2)
    );
    // That's why this function should be defined in the same scope as the struct
    fn area_struct(rectangle: &Rectangle) -> i32 {
        rectangle.width * rectangle.height
    }
    // We can try to print the struct as we always do but this will give us an error,
    // As the struct doesn't implement the Display trait
    // println!("rect2 is {}", rect2);

    // We can try printing it using special syntax :? which will print the struct with output format called "Debug"
    // The Debug trait enables us to print our struct in a way thtat is useful for developer so we can see its value whil we're debugging our code
    println!("rect2 is {:?}", rect2);
    // We still will find an error if we don't include the #[derive(Debug)] annotation before the struct definition
    // After including it, we can print the struct using the :? syntax and the output will be like this:
    // rect2 is Rectangle { width: 30, height: 50 }.
    //? You can also use :#? which will print the struct in a more programmer-friendly way
    println!("rect2 is {:#?}", rect2);

    //? Another way to print out a value using the Debug format is to use dbg! macro
    // Which takes ownership of an expression (as opposed to p[rintln! which takes a reference),
    // And then returns the ownership
    // It will also print the value of the expression along with the file and line number where the dbg! was called
    // As we don't want to take the ownership and we just want to print the value, we can use & to take a reference to the value
    dbg!(&rect2);
}

fn area(width: i32, height: i32) -> i32 {
    width * height
}

fn area_tuple(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}

