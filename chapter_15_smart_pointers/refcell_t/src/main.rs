fn main() {
    // =============================================
    // RefCell<T> and the Interior Mutability Pattern
    // =============================================
    //
    // Interior mutability is a design pattern in Rust that allows you to mutate data even when
    // there are immutable references to that data; normally, this action is disallowed by the
    // borrowing rules. To mutate data, the pattern uses unsafe code inside a data structure to
    // bend Rust's usual rules manually instead of relying on the compiler to check them for us; we
    // will discuss unsafe code more in Chapter 20.
    //
    // We can use types that use the interior mutability pattern only when we can ensure that the
    // borrowing rules will be followed at runtime, even though the compiler can't guarantee that.
    // The unsafe code involved is then wrapped in a safe API, and the outer type is still
    // immutable.
    //
    // Let's explore this concept by looking at the RefCell<T> type that follows the interior
    // mutability pattern.
    //
    // ==============================================
    // Enforcing Borrowing Rules at Runtime with RefCell<T>
    // ==============================================
    //
    // Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it holds. So
    // what makes RefCell<T> different from a type like Box<T>? Recall the borrowing rules you
    // learned in Chapter 4:
    // - At any given time, you can have either one mutable reference or any number of immutable
    // references (but not both).
    // - References must always be valid.
    //
    // With references and Box<T>, the borrowing rules' invariants are enforced at compile time.
    // With RefCell<T>, these invariants are enforced at runtime. With references, if you break
    // these rules, your program will panic and exit.
    //
    // The advantages of checking the borrowing rules at compile time are that errors will be
    // caught sooner in the development process, and there is no impact on runtime performance
    // because all the analysis is completed beforehand. For those reasons, checking the borrowing
    // rules at compile time is the best choice in the majority of cases, which is why this is
    // Rust's default.
    //
    // The advantage of checking the borrowing rules at runtime instead is that certain memory-safe
    // scenarios are then allowed, where they would've been disallowed by the compile-time checks.
    // Statis analysis, like the Rust compiler, is inherently conservative. Some properties of code
    // are impossible to detect by analyzing the code;: the most famous example is the Halting
    // Problem, which is beyond the scope of this book but is an interesting topic to reserach.
    //
    // Because some analysis is impossible, if the Rust compiler can't be sure the code complies
    // with the ownership rules, it might reject a correct program; in this way, it's conservative.
    // If Rust accepted an incorrect program, users wouldn't be able to trust in the guarantees
    // Rust makes. However, if Rust rejects a correct program, the programmer will be
    // inconvenienced, but nothing catastrophic can occur. The RefCell<T> type is useful when
    // you're sure your code follows the borrowing rules but the compiler is unable to understand
    // and guarantee that.
    //
    // Similar to Rc<T>, RefCell<T> is only for use in single-threaded  scenarios and will give you
    // a compile-time error if you try using it in a multithreaded context. We'll talk about how to
    // get the functionality of RefCell<T> in a multithreaded program in Chapter 16.
    //
    // Here is a recap of the reasons to choose Box<T>, RC<T> or RefCell<T>:
    // - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
    // - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only
    // immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows
    // checked at runtime.
    // - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value
    // inside the RefCell<T> even when the RefCell<T> is immutable.
    //
    // Mutating the value inside an immutable value is the interior mutability pattern. Let's look
    // at a situation in which interior mutability is useful and examine how it's possible.
    //
    // ============================================
    // Interior Mutability: A mutable Borrow to an Immutable value
    // ============================================
    //
    // A consequence of the borrowing rules is that when you have an immutable value, you can't
    // borrow it mutably. For example, this code won't compile:
    // let x = 5;
    // let y = &mut x;
    //
    // However, there are situations in which it would be useful for a value to mutate itself in
    // its methods but appear immutable to other code. Code outside the value's methods would not
    // be able to mutate the value. Using RefCell<T> is one way to get the ability to have interior
    // mutability, but RefCell<T> doesn't get around the borrowing rules completely: the borrow
    // checker in the compiler allows this interior mutability, and the borrowing rules are checked
    // at runtime instead. If you violate the rules, you''get a panic! instead of a compiler error.
    //
    // Let's work through a practical example where we can use RefCell<T> to mutate an immutable
    // value and see why that is useful.
    //
    // =============================================
    // A Use Case for Interior Mutability: Mock objects
    // =============================================
    // Sometimes during testing a programmer will use a type in place of another type, in order to
    // observe particular behavior and assert that it's implemented correctly. This placeholder
    // type is called a test double.
    // Think of it in the sense of a stunt double in filmmaking, where a person steps in and
    // substitutees for an actor to do a particularly tricky scene. Test doubles stand in for other
    // types when we're running tests. Mock objects are specific types of test doubles that record
    // what happens during a test so you can assert that the correct actions took place.
    //
    // Rusto doesn't have objects in the same sense as other languages have objects, and Rust
    // doesn't have moock object functionality built into the standart library as some other
    // languages do. However, you can definitely create a struct that will serve the same purposes
    // as mock object.
    //
    // Here's the scenario we'll test: we'll create a library that tracks a value against a maximum
    // value and sends messages based on how close to the maximum value the current value is. This
    // library could be used to keep track of a user's quota for the number of API calls they're
    // allowed to make, for example.
    //
    // Our library will only provide functionality of tracking how close to the maximum a value is
    // and what the messages should be at what times. Applications that use our library will be
    // expected to provide the mechanism for sending the messages: the application could put a
    // message in the application, send an email, send a text message, or do something else. The
    // library doesn't need to know that detail. All it needs is something that implements a trait
    // we'll provide called `Messenger`.
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!")
        }
    }

    // Before continuing, go to the tests.
    //
    // ==============================================
    // Keeping Track of Borrows at Runtime with RefCell<T>
    // ==============================================
    //
    // When creating immmmutable and mutable references, we use the `&` and `&mut` syntax,
    // respectively.
    // With `RefCell<T>`, we use the `borrow()` and `borrot_mut()` smart pointers are currently
    // active. Every time we call `borrow`, the RefCell<T> increases its count of hoy many
    // immutable borows are active. When a Ref<T> value goes out of scope, the count of immutable
    // borrows goes down by 1. Just like the compile-time borrowing rules, RefCell<T> lets us have
    // many immutable borrows or one mutable borrow at any point in time.
    //
    // If we try to violate these rules, rather than getting a compiler error as we would with
    // references, the implementation of `RefCell<T>` will panic at runtime.
    // Next listing shows a modification of the implementation of `send`. We're deliberately trying
    // to create two mutable borrows active for the same scope to illustrate that RefCell<T>
    // prevents us from doing this at runtime.
    //
    // NOTE: Go to tests mod again.
}

// One important part of this code is that `Messenger` trait has one method called `send` that
// takes an immutable reference to `self` and the text of the message. This trait is the interface
// our mock object needs to implement so that the mock can be used in the same way a real object
// is. The other important part is that we want to test the behavior of the `set_value` method on
// the `LimitTracker`. We can change what we pass in for the `value` parameter, but `set_value`
// doesn't return anything for us to make asserions on. We want to be able to say that if we create
// a `LimitTracker` with something that implements the `Messenger` trait and a particular value for
// `max`, when we passs differente numbers for `value`, the messenger is told to send the
// appropiate messages.
//
// We need a mock object that, instead of sending an email or text message when we call `send`,
// will only keep track of the messages it's told to send. We can create a new instance of the mock
// object, create a `LimitTracker` that uses the mock object, call the `set_value` method on
// `LimitTracker`, and then check that the mock object has the messages we expect. Next example
// shows an attempt to implement a mock object to do just that, but the borrow checker won't allow
// it.
//
#[cfg(test)]
mod tests {
    use super::*;

    // struct MockMessenger {
    //     sent_messages: Vec<String>,
    // }
    //
    // impl MockMessenger {
    //     fn new() -> MockMessenger {
    //         MockMessenger {
    //             sent_messages: vec![],
    //         }
    //     }
    // }
    //
    // impl Messenger for MockMessenger {
    //     fn send(&self, message: &str) {
    //         self.sent_messages.push(String::from(message));
    //     }
    // }
    //
    // #[test]
    // fn it_sends_an_over_75_percen_warning_message() {
    //     let mock_messenger = MockMessenger::new();
    //     let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    //
    //     limit_tracker.set_value(80);
    //
    //     assert_eq!(mock_messenger.sent_messages.len(), 1)
    // }

    // We can't modify the `MockMessenger` to keep track of the messages, because the `send` method
    // takes an immutable reference to `self`. We also can't take the suggestion from the error
    // text to use `&mut self` in both the `impl` method trait and the trait definition. We do not
    // want to change the `Messenger` trait solely for the sake of testing. Instead, we need to
    // find a way to make our test code work correctly with our existing design.
    //
    // This is a situation in which interior mutability can help! We'll store the `sent_messages`
    // withit a RefCell<T>, and then the `send` method will be able to modify `sent_messages` to
    // store the messages we've seen. Next listing shows how can we do that.
    //
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }$

    // For first example. (One mutable reference at runtime)
    // impl Messenger for MockMessenger {
    //     fn send(&self, message: &str) {
    //         self.sent_messages.borrow_mut().push(String::from(message))
    //     }
    // }
    //
    // For second example (two mutable references at runtime)
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }

        // We create a variable `one_borrow` for the `RefMut<T>` smart pointer returned from
        // `borrow_mut`. Then we create another mutable borrow in the same way in the variable
        // `two_borrow`. This makes two mutable references in the same scope, which isn't allowed.
        // When we run the tests for our library, the code will compile without any errors, but it
        // will panic at some point.
        //
        // Notice that the code panicked with the message `already borrowed: BorrowMutError`. This
        // is how `<RefCell<T>` handles violations of the borrowing rules at runtime.
        //
        // Choosing to catch borrowing errors at runtime rather than compile time, as we've done
        // here, means you'd potentially be finding mistakes in your code later in the development
        // process: possible not until your code was deployed to production. Also, your code would
        // incur a small runtime performance penalty as a result of keeping track of the borrows at
        // runtime rather than compile time. HOwever, using `RefCell<T>` makes it possible to write
        // a mock object thatn can modify itself to keep track of the messagess it has seen while
        // you're using it in a context where only immutable values are allowed. You can use
        // `RefCell<T>` despite its trade-offs to get more functionality thatn regular references
        // provide. 
        //
        // ===================================================
        // Allowing Multiple Owners of Mutable Data with `Rc<T>` and `RefCell<T>`
        // ===================================================
        // A common way to use `RefCell<T>` is in combination with Rc<T>. Recall that `Rc<T>` lets
        // you have multiple owners of some data, but it only gives immutable access to that data.
        // If you have an `Rc<T>` that holds a `RefCell<T>`, you can get a value that can have
        // multiple owners and that you can mutate!
        //
        // For example, recall the cons list in previous example where we used `Rc<T>` to allow
        // multiple lists to share ownership of another list. Because `Rc<T>` holds only immutable
        // values, we can't change any of the values in the list once we've created them. Let's add
        // in `RefCell<T>` for its ability to change the values in the list. It shows that by using
        // a `RefCell<T>` in the `Cons` definition, we can modify the value stored in all the
        // lists. 
    }

    #[test]
    fn it_sends_an_over_75_percen_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
