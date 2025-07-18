pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn exploration() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
//
//     // Test fails if the code panic!
//     #[test]
//     fn another() {
//         panic!("Make this test fail");
//     }
//
//     // Some other useful macros!
//     // assert! macro to check if some condition is evaluated to true
//     // if is true, the test passed, if false, assert! marco calls panic!
// }

// Let's create a tests module and test Rectangle method
#[cfg(test)] // This is how you define a test module
mod tests {
    // Because the test module is inner module, we need to bring the code under test in the outher
    // module into the scope of the inner module. We need to use a glob here, so anything we define
    // in the outer module is available to this tests module
    use super::*;

    // NOTE: Tests runs in paralell with multiple threads! 
    // If you don't want them to run in paralell as one test depends on another, you should
    // consider using only one thread: "cargo test -- --test-threads=1"
    // 
    // The output of the code from tests will be seen only when test fails, if it passed you won't
    // see the code in terminal, only the message that is passed
    // If you want to see the output of successful test too consider using:
    // "cargo test -- --show-output"
    // NOTE: If you want to run multiple filtered tests:
    // "cargo test <and name or slice of name of tests>"
    // You can also do the same filtering by tests module:
    // cargo test tests
    // NOTE: You can also use #[ignore] attribute after #[test] attribute to ignore a specific test
    // If we want to run only ignore test "cargo test -- --ignored"
    //
    // NOTE: RECOMMENDATION: Consider writing your unit- tests in the same files where you want to test
    // it creating a module inside the file
    // #[cfg(test)] NOTE: cfg stands for configuration and we tell that in this case it should be
    // configured for tests this includes attributes like #[test] or #[ignore] or
    // #[should_panic(<error_message>)]
    // mod tests {}
    // Many programming languages don't allow to test in easy way private functions, in rust
    // private and public functions are test the same 
    // That's why we need to write: use super::*; to bring even private items to the test module
    //
    // NOTE: RECOMMENDATION: While Unit-Tests should be tested inside files of it's functions, 
    // Integration-Tests purpose is to test the integration of multiple files/modules/libraries
    // together.
    // Consider creating a folder tests at the same level as src 
    // Each file in the test is sepparate crate so you need to bring a test into the scope
    //
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            height: 1,
            width: 5,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            height: 1,
            width: 5,
        };
        assert!(!smaller.can_hold(&larger)); // Here we negate the result (reverse it) with ! in
        // the beginning of the parameter
        //
        // You can use assert!() with ==
        // Howeveer, as it is so common to compare two values,
        // You should use:
        // assert_eq! or asser_ne! (passing two parameters to compare),
        // This will compare it and also print to value which makes easier to debug later if
        // something wrong

        // Test value for Guess
    }

    #[test]
    #[should_panic(expected = "value must be less than 100")] // This attribue after the #[test] is expected to panic, if is not panicing wer
    // The expected param should contain a substring of the panic! message 
    // This is recommended as the code could panic by other error, here we specify what panic to
    // expect
    // are getting FAILED
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
        assert_eq!(add_two(4), 6);
        assert_eq!(add_two(2), result)
    }

    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // This is how to print the custom error message
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, values was {result}"
        );
    }

    // Test so they return a Result<T, E> enables you to use the question mark operator in the body
    // of tests which can be a convenient way to write tests that should fail if any operation
    // within them returns an Err variant
    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello!")
}

// In addition to checkin return values, it's important to check that our code handles error
// conditions as we expect. For example:
struct Guess {
    value: i32,
}

impl Guess {
    // pub fn new(value: i32) -> Guess {
    //     if value < 1 || value > 100 {
    //         panic!("Guess value must be between 1 and 100, got {value}")
    //     }
    //
    //     Guess { value }
    // }

    pub fn new(value: i32) -> Guess {
        if value > 100 {
            panic!("Guess value must be less than 1000")
        }

        if value < 1 {
            panic!("Guess value must be greater or equal to 1")
        }
        Guess { value }
    }
}
