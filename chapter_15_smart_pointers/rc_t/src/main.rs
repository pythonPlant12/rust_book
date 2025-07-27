fn main() {
    // In the majority of cases, ownership is clear: you know exactly which variable owns a given
    // value.
    // However, there are cases when a signle value might have multiple owners. For example, in
    // graph data structures, ,ultiple edges might point to the same node, and that node is
    // conceptually owned by all of the edges that point to it. A node shouldn't be cleaned up
    // unless it doesn't have any edges pointing to it and so has no owners.
    //
    // You have to enable multiple ownership explicitly by using the Rust type Rc<T>, which is an
    // abbreviation for reference counting. The Rc<T> type keeps track of the number of references
    // to a value to determine whether or not the value is still in use. If there are zero
    // references to a value, the value can be cleaned up without any references becoming invalid.
    //
    // Imagine Rc<T> as a TV in a family room. When one person enters to watch TV, they turn it on.
    // Others can come into the room and watch the TV. When the last person leaves the room,  they
    // turn off the TV because it's no longer being used. If someone turns off the TV while others
    // are still watching it, there would be an uproar from the remaining TV watchers!
    //
    // We use the RC<T> type when we want to allocate some data on the heap for multiple parts of
    // our program to read and we can't determine at compile time which part will finish using the
    // data last. If we knew which part would finish last, we could just make that part the data's
    // owner, and the normal owneship rules enforced at compile time would take effect.
    //
    // Note that Rc<T> is only for use in signle-threaded scenarios. When we discuss concurrency in
    // Chapter 16, we'll cover how to do reference counting in multithreaded programs.
    //
    // ==============================
    // Using Rc<T> to Share Data
    // ==============================
    // Let's return to our cons list example. Recall that we defined it using Box<T>. This time,
    // we'll create two lists that both share ownership of a third list.
    // We'll create list a that contains 5 and then 10. Then we'll make two more lists: b that
    // starts with 3 and c that starts with 4. Both b and c lists will then continue on to the
    // first a list containing 5 and 10. In other words, both lsits will share the first list
    // containing 5 and 10.
    //
    // Trying to implement this scenario using our definition of List with Box<T> won't work.
    // enum List {
    //     Cons(i32, Box<List>),
    //     Nil
    // }
    //
    // enum List {
    //     Cons(i32, Rc<List>),
    //     Nil,
    // }

    // let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
    // let b = List::Cons(3, Box::new(a));
    // let c = List::Cons(4, Box::new(a));

    // This will not compile because the Cons variants own the data they hold, so when we create
    // the b list, a is moved into b and b owns a. Then, when we try to use a again when creatintg
    // c, we're not allowed to because a has been moved.
    //
    // We could change the definition of Cons to hold references instead, but then we would have to
    // specify life5time parameters. By specifying lifetime parameters, we would be specifying that
    // every element in the list will live at least as long as the entire list. This is the case
    // for the elements and lists in this example that could work, but not in every scenario.
    //
    // Instead, we'll change our definition of List to use Rc<T> in place of Box<T>. Each Cons
    // variant will now hold a value and an Rc<T> poinint to a List. When we create b, instead of
    // taking ownership of a, we'll clone the Rc<List> that a is holding, thereby increasing the
    // number of references from one to two and letting a and b share ownership of the data in that
    // Rc<List>. We'll also clone a when creating c, increasing the number of references from two
    // to three. Every time we call Rc::clone, the refereence count to the data withit the Rc<List>
    // will increase, and the data won't be cleaned up unlesss there are zero references to it.
    // We need to add use Rc because it is not in prelude, but in standart library
    //
    // use std::rc::Rc; // Rc stands for - reference counting
    // let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    // let b = List::Cons(3, Rc::clone(&a));
    // let c = List::Cons(4, Rc::clone(&a));

    // The Rc::clone() method is not the same as clone() of deep copy on other variables. Rc::clone
    // only incremenets the reference count, which doesn't take much time. Deep copies of data can
    // take a lot of time. By using Rc::clone for reference counting, we can visually distinguish
    // between the deep-copy kinds of clones and the kinds of clones that increase the reference
    // count. When looking for performance problems in the code, we only need to consider the
    // deep-copy clones and can disregards calls to Rc::clone.
    //
    // Le'ts change our working example so we can see the reference counts changing as we create
    // and drop references to the Rc<List> n a.
    //
    //
    // use std::rc::Rc; // Rc stands for - reference counting
    // let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    // println!("count after creating a = {}", Rc::strong_count(&a));
    // let b = List::Cons(3, Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(Rc::clone(&a)))))) );
    // println!("count after creating b = {}", Rc::strong_count(&a));
    // {
    //     let c = List::Cons(4, Rc::clone(&a));
    //     println!("count after creating c = {}", Rc::strong_count(&a));
    // }
    // println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    //
    //
    // We can see that the Rc<List> in a has an initial reference count of 1; then each time we call clone, 
    // the count goes up by 1. When c goes out of scope, the count goes down by 1. 
    // We don’t have to call a function to decrease the reference count like we have to call Rc::clone to increase the reference count: 
    // the implementation of the Drop trait decreases the reference count automatically when an Rc<T> value goes out of scope.
    //
    // What we can’t see in this example is that when b and then a go out of scope at the end of main, 
    // the count is then 0, and the Rc<List> is cleaned up completely. Using Rc<T> allows a single value to have multiple owners, 
    // and the count ensures that the value remains valid as long as any of the owners still exist.
    //
    // Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only. 
    // If Rc<T> allowed you to have multiple mutable references too, you might violate one of the borrowing rules discussed in Chapter 4: 
    // multiple mutable borrows to the same place can cause data races and inconsistencies. 
    // But being able to mutate data is very useful! In the next section, we’ll discuss the interior mutability pattern and the RefCell<T> 
    // type that you can use in conjunction with an Rc<T> to work with this immutability restriction.
    //
    //
    // NOTE: This is example for `RefCell<T>` from  the Chapter 15.5 
    //
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil
    }

    use List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
