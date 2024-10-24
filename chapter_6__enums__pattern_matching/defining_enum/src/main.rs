fn main() {
    // When structs give you a way of grouping together related fields and data,
    // enums give you a way of saying a value is one of a possible set of values.
    // We can express this concept in code
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }
    // Now we can create instances of each of the two variants of IpAddrKind
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Now we can define a function which takes a parameter of type IpAddrKind
    fn route(ip_addr_kind: IpAddrKind) {}
    route(four);
    route(six);

    // Now we can use enums to define a struct fields
    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home: IpAddr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback: IpAddr = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    dbg!(&home);
    dbg!(&loopback);

    // However, right now, we just say what will be the type of enum but not associated any value with an enum
    // We can associate values with enum variants
    #[derive(Debug)]
    enum IpAddrKind1 {
        V4(String),
        V6(String),
    }
    struct IpAddr1 {
        kind: IpAddrKind1,
        address: String,
    }

    let home1 = IpAddr1 {
        kind: IpAddrKind1::V4(String::from("hi")),
        address: String::from("127.0.0.1"),
    };
    let loopback1 = IpAddr1 {
        kind: IpAddrKind1::V6(String::from("::hi")),
        address: String::from("::1"),
    };
    let home2 = IpAddrKind1::V6(String::from("::1"));
    dbg!(&home2);

    // With enums, we attach data to each variant of enum directly, so there is no need for an extra struct,
    // The way it works is that when we call the enum() variant, it becomes a function that constructs an instance of the enum
    // So when we call IpAddrKind1::V6(...) it construct an instance of a variant of IpAddrKind1 and returns an instance of this type

    // There's another advantage of using enums vs structs, we can store multiple values on the each variant of enum
    #[derive(Debug)]
    enum IpAddrKind3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    #[derive(Debug)]
    struct IpAddr3 {
        kind: IpAddrKind3,
        address: String,
    }
    let home3 = IpAddr3 {
        kind: IpAddrKind3::V4(127, 0, 0, 1),
        address: String::from("::1"),
    };
    let loopback3 = IpAddr3 {
        kind: IpAddrKind3::V6(String::from("::1")),
        address: String::from("::1"),
    };
    dbg!(&home3);
    dbg!(&loopback3);

    // If you have a look to std library, it has it's own implementation for IpAddresses,  however, it is a bit different
    // In their case, the varians of enums receive a struct as a parameter, so it means enum varians can be different data types,
    // Even another enum, e.g
    struct Ipv4Addr {
        // --snip--
    }
    struct Ipv6Addr {
        // --snip--
    }
    enum IpAddrStd {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    // Let's look at another example of an enum
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    // This is similar to do it with structs
    struct Quit;
    struct Move {
        x: i32,
        y: i32
    }
    struct Write(String); // tuple struct
    struct ChangeColor(i32, i32, i32); // tuple struct

    // But if we'd use structs, each of which has its own type, we couldn't as easily define a function
    // to take any of these kinds of messages as we could with Message enum

    // Here is another similarity with structs, we can defined methods on enums with impl
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();
    // The body of the method would use self to get the value that we called the method on.
    // In this example, we've created a variable m that has the value Message::Write(String::from("hello"));
    // and that is what self will be in the body of the call method when m.call() runs.

    // Let's take another example of std library Option<T> enum
    // This enum covers a common scenario where the value could be something or it could be nothing
    // There are many cases where you can have null, however, it may lead to many errors when you try
    // to access a null value, Rust doesn't have null values, but it does have an enum that can encode the
    // concept of a value beign present or absent. This enum is Option<T> and it is in std library
    enum Option<T> {
        None,
        Some(T)
    }
    // The Option<T> is so common so that it was included in the prelude, you don't need to bring
    // it into the scope explicitly. It's values also included in the prelude, so you don't need to use,
    // Option::... . Option<T> is still a regular enum
    // The <T> syntax is a generic type parameter, it means the Some variant of the Option enum can hold one value of any type
    // We'll cover generics in Chapter 10. The <T> type inherits the type of the value that will be held by the Some variant of the Option enum
    let some_number = Some(5);
    let some_string = Some("String");
    // let absent_number = None; //This will give as an error as None variant is not of type i32

    // This code will not compile
    let x: i8 = 5;
    // let y: Option<i8> = Some(5); // Some(5) is not of type Option<i8> rather std::option::Option<i8>
    // Option<T> and T (i8) are different types, so Rust won't compile next code
    // let y: Option<i8> = Some(5); // This will compile
    // Neither this will compile
    // let sum = x + y; // As Rust doesn't know how to sum Option type

    // With Option<T> we avoid to use null values, and we'll must handle the case when the value is null (None)
    // So everywhere where the value is not Option<T> you can be 100% sure that the value is not null
}
