pub trait Summary {
    fn summarize(&self) -> String;
    // We can also directly write the default behavior for this function in trait
    // fn summarize(&self) -> String {
    //     "Summarizable".to_string()
    // }
    // To use only default methods without need to write your own
    // You should write somethings like impl Summary for NewsArticle {} with empty body
    //
    // We can also use those methods which are not defined yet in the trait from another method
    // fn summarize_author(&self) -> String;
    // fn summarize(&self) -> String {
    //     format!("Author: {self.summarizable_author()}")
    // }
    // In this case, on implementing this trait, we'll only need to define the summarize_author
    // method
}
