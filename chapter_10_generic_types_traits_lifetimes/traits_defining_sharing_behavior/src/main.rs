pub mod aggregator;
use std::fmt::Display;

use aggregator::Summary;

fn main() {
    // Now we can implement this trait (shared behavior to different structs)
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {}, ({})", self.headline, self.author, self.location)
        }
    }

    pub struct SocialPost {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub repost: bool,
    }

    impl Summary for SocialPost {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    let post = SocialPost {
        username: "username_value".to_string(),
        content: "content_value".to_string(),
        reply: false,
        repost: false,
    };
    println!("1 new social post: {}", post.summarize());

    // Take into consideration that if the Trait is written in another module/crate, it should be
    // public if you want to use it, as well as when you want to use this method of the trait you
    // should import it too: use aggregator::{SocialPost, Summary};
    //
    //
    trait TestTrait {
        fn default_function(&self) -> String {
            "Default_function".to_string()
        }

        fn custom_function(&self) -> String;
    }

    struct TestStruct {}

    impl TestTrait for TestStruct {
        fn custom_function(&self) -> String {
            self.default_function()
        }
    }
    let instance_test_struct = TestStruct {};
    println!("{}", instance_test_struct.custom_function());

    // When we define a method, we can say: you can accept all the types that implements this
    // trait.
    fn fn_with_param_of_this_trait(item: &impl Summary) -> String {
        // Thanks to this &impl Summary, we don't know what type we'll receive, however we can
        // access all the methods of this trait as the function will only accept the type which
        // implemented the trait Summary before
        item.summarize()
    }
    // The implementation of &impl Summary as param type on the method is sugar syntax for:
    // "trait bound syntax", the previous is the same as this:
    fn fn_with_param_generic_param_trait<T: Summary>(item: &T) {};

    // Sometimes is convenient to use one form, while in other cases, other.
    // Example:
    fn notify(item1: &impl Summary, item2: &impl Summary) {}
    fn notify1<T: Summary>(item1: &T, item2: &T) {}
    fn notify2<T: Summary, Y: Summary>(item1: &T, item2: &Y) {}
    // As you can see in this case the shorter is the second variant
    //
    // You can also combine trait to define generic and type
    fn notify3<T: Summary + TestTrait>(item: &T) {}
    fn notify4<Y: Summary, T: Summary + TestTrait>(item1: &Y, item2: &T) {}
    fn notify5(item1: &(impl Summary + TestTrait), item2: &impl Summary) {}

    // Sometimes it may be hard to read when multiple generics and params in a function have multiple trait
    // bound. In this case you can use "where" syntax
    fn notify6<T, U>(item1: &T, item2: &U) -> ()
    where
        T: Display + Clone,
        U: Summary + TestTrait,
    {
    }

    // We can also specify returning type with a specific trait
    fn notify7() -> impl TestTrait {
        TestStruct {}
    }

    // It is also possible to make a condition depengin of what type is generic type coming from
    // the parameter. Here is the example:
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd>  Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x)
            } else {
                println!("The largest member is y = {}", self.y)
            }
        }
    }

    let pair_with_number = Pair {x: 1, y: 2};
    let pair_with_str = Pair { x: "hi", y: "hello" };
    pair_with_number.cmp_display();
    pair_with_str.cmp_display();
    let pair_with_vec = Pair { x: vec![1], y: vec![] };
    // This now will fail as there is no Display/PartialOrd traits implemented for type Vec<T>
    // pair_with_vec.cmp_display();
}
