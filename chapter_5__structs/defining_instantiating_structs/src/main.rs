fn main() {
    // Example of defining a struct with values
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // Example of initializing a struct (object)
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@gmail.com"),
        sign_in_count: 5,
    };

    // To get a specific value of struct use dot notation
    println!("{}", user1.username);

    // You can change the value of a struct, but the struct must be mutable
    user1.active = false;

    //? In Rust, you cannot define mutable and immutable fields in a struct, you can only define mutable struct
    // As with any expression, we can construct the struct and which will return the instance of this struct
    // Example of constructing a struct with a function
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            email,
            username,
            sign_in_count: 0,
        }
    }
    // You can use user1 as a template to create a new user providing field by field or the rest
    // of the fields from user1
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("nikitalutsai@gmail.com"),
        sign_in_count:user1.sign_in_count
    };
    // In this case you provide all the fields from user1
    let user3 = User {
        ..user2
    };
    // Take into consideration that after asigning user1 to user2, user1 assigned fields are not available anymore
    // As we "moved" the ownership of the fields to user2
    // let user3 = User {
    //     ..user2
    // };


    // We can also define struct without named fields, but in this case we need to provide the fields in the same order
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // We can access the variables by order as by standart tuple, like black.0

    // We can also create unit-like structs, which are structs without any fields
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    // println!("{:?}", subject);

    // Take into consideration that we can also define &str instead of String,
    // but in this case we need to provide lifetimes, this is something we will see in Chapter 10
    // But normally we want instances to own it's data
    #[derive(Debug)]
    struct Prueba1 {
        mail: String,
    }
    let prueba1_instance = Prueba1 {
        mail: String::from("prueba1"),
    };
    let prueba1_instance2 = Prueba1 {
       prueba1_instance
    };
    dbg!(prueba1_instance);
    dbg!(prueba1_instance2);

}

