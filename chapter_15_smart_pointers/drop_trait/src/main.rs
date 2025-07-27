fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff")
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff")
    };

    println!("CustomSmartPointers created.");
    // However, if you want to explicitly drop a variable before it goes out of scope,
    // you should call the drop() method.
    // This will give us an error because of destructor.
    // Desctructor is similar to constructor.
    // It drops instances when they go out of scope automatically.
    // We cannot disable it and rust is not giving as an option of doing it, otherwise,
    // we'll make an error of dropping the same instance twice.
    // We can't disable automatic insertion of drop when value goes out of scope.
    // And we can't call the method drop() explicitly.
    //
    // If we really need to force drop the value to be cleared before it goes out of scope.
    // We need to use std::mem::drop function.
    // This function is in prelude.
    // c.drop(); // Won't work
    drop(c); // This will work
    // This will call the drop method from Drop trait before going out of scope.
    // As said before, the drop trait removes instances in reverse order.
    // However, in this case it will remove first the c variable than d.
    println!("CustomSmartPointer dropped before the end of main");
    
    // The good thing than even you can implement your own memory allocator. The rust owneship
    // system will always check that the value you are using are always valid and you don't need to
    // think about cleaning up variables because rust will do it automatically through its system.
}

struct CustomSmartPointer {
    data: String
}

// Drop trait comes in prelude so we don't need to import it.
// It will drop the references or variables that goes out of scope by default
// However, we can implement it in case we need to add some more functionality.
// Note that we didn't need to call the drop method explicitly.
// Here even we override drop method from Drop trait, rust will automatically manage memory through
// its ownership system.
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: `{}` !", self.data);
    }
}
