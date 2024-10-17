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
}

fn area(width: i32, height: i32) -> i32 {
    width * height
}

fn area_tuple(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}

