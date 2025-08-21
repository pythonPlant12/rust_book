use crate::blog::Post;

fn main() {
    let mut post = Post::new();
    // The issue is that here, even the post is draft, we are still able to add text to it.
    // But instead of having a `content` method on a draft post that returns an empty string, we'll
    // make it so draft posts don't have the content method at all. That way, if we try to get a
    // draft post's content, we'll get a compiler error telling us the method doesn't exits. As a
    // result, it will be impossible for us to accidentally display draft post content in
    // production because that code won't even compile.
    //
    // Go to oop_design_patterns_1 to check the second approach
    post.add_text("I ate a salad for lunch today!");
    assert_eq!("", post.content())
}

pub mod blog {
    #[derive(Default)]
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        // For now we want to return empty string (before adding control of the state)
        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        pub fn aprove(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }

        pub fn reject(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.reject())
            }
        }
    }

    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;

        fn approve(self: Box<Self>) -> Box<dyn State>;

        fn content<'a>(&self, _post: &'a Post) -> &'a str {
            ""
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {})
        }
    }

    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    struct PendingReview {}

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }
}
