fn main() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        // This is method as is a function of a instance
        fn area(&self) -> u32 {
            self.width * self.height
        }
        // This is an associated function to a Struct
        // but it's not a method as it doesn't take a reference to an instance
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
        // We can pass parameters as in any function
        fn can_hold(&self, other: &Rectangle) -> bool {
            (self.width * self.height) > (other.width * other.height)
        }
    }
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    let rect2 = Rectangle::square(3);
    let rect3: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };

    dbg!(&rect2);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect3));
}

