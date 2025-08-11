fn main() {
    pub trait Draw {
        fn draw(&self);
    }

    // In this case, the screen Struct can have many objects of different types that implements
    // Draw trait
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    // This works differently from defining a struct that uses a generic type parameter with trait
    // bounds. A generic type parameter can be substituted with only one concrete type at a time,
    // whereas trait object allow for multiple concrete types to fill in for the trait object at
    // runtime. For example, we could have defined the `Screen` struct using a generic type and a
    // trait bound:
    pub struct Screen1<T: Draw> {
        pub components: Vec<T>,
    }
    impl<T> Screen1<T>
    where
        T: Draw,
    {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw()
            }
        }
    }
    // This restricts us to a `Screen1` instance that has a list of components all of type `Button`
    // or all of type `TextField`. If you'll only ever have homogeneous collections, using generics
    // and trait bounds is preferable because the definitions will be monomorphized at compile time
    // to use the concrete types.
    //
    // On the other hand, with the method using trait objects, one `Screen` instance can hold a
    // `Vec<T>` that contains a `Box<Button>` as well as a `Box<TextField>`. Let's look at how this
    // works, and then we'll talk about the runtime performance implications.
    //
    // ** Implementing the Trait **
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            // code that actually draw a button
            println!("Drawing button");
        }
    }
    //
    // The `width`, `height`, `label` fields on `Button` will differ from the fields on toher
    // components; for example, a `TextField` type might have those same fields plus a `placeholder` field.
    // Each of the types we want to draw on the screen will implement the `Draw` trait but will use
    // different code in the `draw` method to define how to draw that particula type, as `Button`
    // has here (without the actual GUI code, as mentioned). The `Button` type, for instance, might
    // have an additional `impl` block containing methods related to what happens when a user
    // clicks the button. These kinds of methods won't apply to types like `TextField`.
    //
    // If someone using our library decides to implement a `SelectBox` struct that has `width`,
    // `height` and `options` fields, they would implement the `Draw` trait on the `SelectBox` type
    // as well.
    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }
    impl Draw for SelectBox {
        fn draw(&self) {
            // code to actually draw a select box
            println!("Drawing SelectBox");
        }
    }


    // Now we can create `Screen` instance where components are with different types:
    let screen = Screen {
        components: vec![
            Box::new(
            SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("yes"),
                        String::from("no"),
                    ]
                }
            ),
            Box::new(
                Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK")
                }
            )
        ]
    };
    screen.run()
}
