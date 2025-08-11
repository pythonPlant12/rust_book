fn main() {
    // Ojbect-oriented programs are made up of objects. An objects packages
    // both data and the procedures that operate on that data. The procedures are typically called
    // methods or operations.
    //
    // Using this definition, Rust is object oriented: structs and enums have data, and `impl`
    // blocks provide methods on structs and enums. Even though structs and enums with methods
    // aren't called objects, they provide the same functionality, according to the Gangof Four's
    // definition of objects.
    //
    // ** Encapsulation **
    //
    // Another aspect commonly associated with OOP is the idea of `encapsulation`, which means that
    // the implementation details of an object aren't accessible to code using that object.
    // Therefore, the only way to interact with an object is throught its public API; code using
    // the object shouldn't be able to reach into the object's internals and change data or
    // behavior directly. This enables the programmer to change and refactor an object's internals
    // without needing to change the code that uses this object.
    //
    // We can use the `pub` keyword to decide which modules, types, functions, and methods in our
    // code should be public, and by default everything else is private. The following struct can
    // also have a field that contains the average of the values in the vecor, meaning the average
    // doesn't have to be computed on demand whenever anyone needs it. In other words, the
    // following struct will cache the calculated average for us.
    //
    #[allow(dead_code)]
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    // The struct is marked `pub` so that other code can use it, but the fields within the struct
    // remain private. This is important in this case because we want to ensure that whenever a
    // value is added or removed from the list, the average is also updated. We do this by
    // implementing add, remove, and average methods on the struct.
    impl AveragedCollection {
        #[allow(dead_code)]
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        #[allow(dead_code)]
        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }

        #[allow(dead_code)]
        pub fn average(&self) -> f64 {
            self.average
        }

        #[allow(dead_code)]
        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.iter().len() as f64
        }

        // The public methods `add`, `remove`, `average` are the only ways to access or modify data
        // in an instance of AverageCollection. When an item is added to `list` using the `add`
        // method or removed using the `remove` method, the implementation of each call the private
        // `update_average` method that handles updating the `average` field as well.
        //
        // We leave the `list` and `average` fields private so there is no way for external code to
        // add or remove items to or from the `list` field directly; otherwise, the `average` field
        // might become out of sync when the `list` changes. The `average` method returns the value
        // in the `average` field, allowing external code to read the `average` but not modify it (like getter)
        //
        // Because we'vbe ecapsulated the impolementation details of the struct `AveragedCollection`,
        // we can easily change aspects, such as the data structure, in the future.
        // For instance, we could use a `HashSet<i32>` instead of a `Vec<i32>` for the `list` field.
        // As long ass the signatures of the `add`, `remove`, and `average` public methods stayed
        // the same, code using `AveargedColection` wouldn't need to change. If we made `list`
        // public instead, this wouldn't necessarily be the case: `HashSet<i32>` and `Vec<i32>`
        // have different methods for adding and removing items, so the external code would likely
        // have to change if it were modifyint `list` directly.
        //
        // If encapsulation is a required aspect for a language to be considered object oriented,
        // then Rust meets that requirement. The option to use `pub` or not for different parts of
        // code enables encapsulation of implementation details.
        //
        // ** Inheritation as a Type System and as Code Sharing **
        // Inheritance is a mechanism whereby an object can inherit elements from another object's
        // definition, thus gaining the parent object's data and behavior without you having to
        // define them again.
        //
        // If a language must have inheritance to be object oriented, then Rust is not such a
        // language. There is no way to define a struct that inherits the parent strtuct's fields
        // and method implementations without using a macro.
        //
        // However, if you're used to having inheritance in your programming toolbox, you can use
        // other solutions in Rust, depending on your reason for reaching for inheritance in the
        // first place.
        //
        // You would choose inheritance for two main reasons. One is for reuse of code: you can
        // implement particular behavior for one type, and inheritance enables you to reuse that
        // implementation for a different type. You can do this in a limited way in Rust code using
        // default trait method implementations. And type that implements the same trait will have
        // the shared behavior. This is similar to a parent class having an implementation of a
        // method and an inheriting child class also having the implementation of the method. We
        // can also override the default implementation of this method from the trait, which is
        // similar to a child class overriding the implementation of a method inherited from a
        // parent class.
        //
        // The other reason to use inheritance relates to the type system: to enable a child type
        // to be used in the same places as the parent type. This is also called polymorphism,
        // which means that you can substitute multiple objects for each other at runtime if they
        // share cerain characteristics.
        //
        // ** Polymorphism **
        // To many people, polymorphism is synonymous with inheritance. But it's actually a more
        // general concept that refers to code that can word with data of multiple types. For
        // inheritance, those types are generally subclasses.
        //
        // Rust instead uses generics to abstract over different possible types and trait bounds to
        // impose constraints on what those types must provide. This is sometimes called `bounded
        // parametric polymorphism`.
        // Inheritance has recently fallen out of favor as a programming design solution in many
        // programming languages because it's often at risk of sharing more code than necessary.
        // Subclasses shouldn't always share all characteristics of their parent class but will do
        // so with inheritance. This can make a program's design less flexible. It also introduces
        // the possibility of calling methods on subclasses that don't make sense or that cause
        // errors because the methods don't apply to the subclass. In addition, some languages will
        // only allow single inheritance (meaning a subclass can only inherit from one class),
        // further restricting the flexility of a program's design.
        //
        // For these reasons, Rust takes the different approach of using trait objects instead of
        // inheritance. Let's look at how trait objects enable polymorphism in Rust.
    }
}
