use std::{cell::RefCell, rc::Rc};
use crate::List::{Cons, Nil};


fn main() {
    // Rust's memory safety guarantees make it difficult, but not impossible, to accidentally
    // create memory that is never cleaned up (known as a memory leak). Preventing memory leaks
    // entirely is not one of Rust's guarantees, meaning memory leaks are memory safe in Rust. We
    // can see that Rust allows memory leaks by using `Rc<T>` and RefCell<T>: it's possible to
    // create references where items refer to each other in a cycle. This creates memory leaks
    // because the reference count of each item in the cycle will never reach 0, and the values
    // will never be dropped.
    //
    // =====================================
    // Creating a Reference Cycle 
    // =====================================
    //
    // Let's look at how a reference cycle might happen and how to prevent it, starting with the
    // definition of the `List` enum and a `tail` method.
    //
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count: {}", Rc::strong_count(&a)); // Strong count allows us to know how
    // many owners are for this list
    println!("a next item: {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation: {}", Rc::strong_count(&a));
    println!("b initial rc count: {}", Rc::strong_count(&b));
    println!("b next item: {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a: {}", Rc::strong_count(&b));
    println!("a rc count after changing a: {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    println!("a next item: {:?}", a.tail());

    // We create an `Rc<List>` instance holding a `List` value in the variable `a` with an initial
    // list of `5, Nil`. We then create an `Rc<List>` instance holding another `List` value in the
    // variable `b` that contains the value `10` and points to the list in `a`.
    //
    // We modify `a` so it points to `b` instead of `Nil`, creating a cycle. We do that by using
    // the `tail` method to get a reference to the `RefCell<Rc<List>>` in `a`, which we put in the
    // variable `link`. Then we use the `borrow_mut` method on the `RefCell<Rc<List>>` to change
    // the value inside from an `Rc<List>` that holds a `Nil` value to the `Rc<List>` in `b`.
    //
    // When we run this code, keeping the last println! commented out for the moment, we'll get
    // output without errors.
    // However, if we uncomment it, we'll get the `stack overflow` error. 
    // As when the main function finishes, it drops the pointers &a and &b, however, as we have
    // multiple strong references from Rc::, there will be reference count > . 
    // So the code could break during last printing due to infinite recursion.

}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}
// We're using another variation of the `List` definition from previous Listing. The second element
// in the Cons variant is now RefCell<Rc<List>>, meaning that instead of having the ability to
// modify the i32 value as we did in previous Listing (antother file), we want to modify the `List`
// value a Cons variant is pointing to. We're also adding a `tail` method to make it convenient for
// us to access the second item if we have a `Cons` variant
//
// In a next listing, we're adding a `main` function that uses the definitions from previous
// listing. This code creates a list in `a` and a list in `b` that points to the list `a`. Then it
// modifies the list in `a` to point to `b`, creating a reference cycle. There are `println!`
// statements along the way to show what the reference counts are at various points in this
// process.
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
// ===================================================
// NOTE: Review of this code Step-by-Step regarding Ownership/Memory Management
// ===================================================
//
//# Rust Reference Cycle Analysis

// ## Your Comments Review
//
// ✅ **Correct observations:**
// - Memory leaks are possible in Rust and are considered "memory safe"
// - Reference cycles prevent values from being dropped
// - The cycle is created when `a` points to `b` and `b` points to `a`
// - Uncommenting the last `println!` causes stack overflow
//
// ❌ **Minor corrections needed:**
// - The stack overflow happens during printing (infinite recursion), not due to "memory crash"
// - It's not exactly "1 pointer left for each" - it's that both have reference count > 0
//
// ## Step-by-Step Memory and Ownership Analysis
//
// ### Step 1: Creating `a`
// ```rust
// let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
// ```
//
// **What happens in memory:**
//
// **Stack:**
// ```
// main() stack frame:
// ├── a: Rc<List> (pointer to heap location H1)
// ```
//
// **Heap:**
// ```
// H1: Rc control block { 
//     strong_count: 1, 
//     data: Cons(5, RefCell { 
//         value: Rc<List> (pointer to H2) 
//     }) 
// }
//
// H2: Rc control block { 
//     strong_count: 1, 
//     data: Nil 
// }
// ```
//
// **Ownership:** `a` owns an `Rc` that points to the `Cons(5, ...)` on the heap.
//
// ---
//
// ### Step 2: Print initial counts
// ```rust
// println!("a initial rc count: {}", Rc::strong_count(&a)); // Prints: 1
// println!("a next item: {:?}", a.tail());                  // Prints: Some(RefCell { value: Nil })
// ```
//
// **Memory state:** Unchanged. We're just reading the reference count and the tail.
//
// ---
//
// ### Step 3: Creating `b`
// ```rust
// let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
// ```
//
// **Stack:**
// ```
// main() stack frame:
// ├── a: Rc<List> (pointer to H1)
// ├── b: Rc<List> (pointer to H3)
// ```
//
// **Heap:**
// ```
// H1: Rc control block { 
//     strong_count: 2,  // ← INCREASED! (a + the clone in b)
//     data: Cons(5, RefCell { 
//         value: Rc<List> (pointer to H2) 
//     }) 
// }
//
// H2: Rc control block { 
//     strong_count: 1, 
//     data: Nil 
// }
//
// H3: Rc control block { 
//     strong_count: 1, 
//     data: Cons(10, RefCell { 
//         value: Rc<List> (pointer to H1)  // ← Points back to a!
//     }) 
// }
// ```
//
// **Key insight:** `Rc::clone(&a)` increments the reference count of `a` from 1 to 2.
//
// ---
//
// ### Step 4: Print counts after b creation
// ```rust
// println!("a rc count after b creation: {}", Rc::strong_count(&a)); // Prints: 2
// println!("b initial rc count: {}", Rc::strong_count(&b));          // Prints: 1
// println!("b next item: {:?}", b.tail());                           // Prints: Some(RefCell { value: Cons(5, ...) })
// ```
//
// **Current structure:**
// ```
// a → Cons(5, RefCell(Rc → Nil))
// b → Cons(10, RefCell(Rc → a))
// ```
// This is NOT a cycle yet! `b` points to `a`, but `a` still points to `Nil`.
//
// ---
//
// ### Step 5: Creating the cycle!
// ```rust
// if let Some(link) = a.tail() {
//     *link.borrow_mut() = Rc::clone(&b);
// }
// ```
//
// **Breaking this down:**
// 1. `a.tail()` returns `Some(&RefCell<Rc<List>>)` - a reference to the RefCell inside `a`
// 2. `link.borrow_mut()` gets a mutable reference to the `Rc<List>` inside the RefCell
// 3. `*link.borrow_mut() = Rc::clone(&b)` replaces the old `Rc<List>` (pointing to Nil) with a new `Rc<List>` (pointing to `b`)
//
// **Heap after this operation:**
// ```
// H1: Rc control block { 
//     strong_count: 2, 
//     data: Cons(5, RefCell { 
//         value: Rc<List> (pointer to H3)  // ← NOW POINTS TO b!
//     }) 
// }
//
// H2: Rc control block { 
//     strong_count: 0,  // ← DROPPED! No longer referenced
//     data: Nil         // ← This gets cleaned up
// }
//
// H3: Rc control block { 
//     strong_count: 2,  // ← INCREASED! (b + the clone in a)
//     data: Cons(10, RefCell { 
//         value: Rc<List> (pointer to H1)
//     }) 
// }
// ```
//
// **NOW WE HAVE A CYCLE:**
// ```
// a → Cons(5, RefCell(Rc → b))
// b → Cons(10, RefCell(Rc → a))
// ```
//
// ---
//
// ### Step 6: Final counts
// ```rust
// println!("b rc count after changing a: {}", Rc::strong_count(&b)); // Prints: 2
// println!("a rc count after changing a: {}", Rc::strong_count(&a)); // Prints: 2
// ```
//
// Both `a` and `b` now have reference count 2 because:
// - `a` has count 2: one from variable `a`, one from the Rc inside `b`
// - `b` has count 2: one from variable `b`, one from the Rc inside `a`
//
// ---
//
// ### Step 7: The stack overflow
// ```rust
// println!("a next item: {:?}", a.tail());
// ```
//
// This tries to print the Debug representation of the entire structure. The Debug implementation recursively tries to print:
// - `a` contains a RefCell with `b`
// - `b` contains a RefCell with `a`  
// - `a` contains a RefCell with `b`
// - `b` contains a RefCell with `a`
// - ... (infinite recursion)
//
// Each recursive call adds a new frame to the call stack until it overflows.
//
// ---
//
// ### Step 8: When main() ends
//
// **Stack:**
// ```
// main() stack frame ends:
// ├── a: Rc<List> goes out of scope → strong_count(H1) decreases to 1
// ├── b: Rc<List> goes out of scope → strong_count(H3) decreases to 1
// ```
//
// **Heap:**
// ```
// H1: Rc control block { 
//     strong_count: 1,  // ← Still has reference from H3!
//     data: Cons(5, RefCell { value: Rc<List> (→ H3) }) 
// }
//
// H3: Rc control block { 
//     strong_count: 1,  // ← Still has reference from H1!
//     data: Cons(10, RefCell { value: Rc<List> (→ H1) }) 
// }
// ```
//
// **The Problem:** Both objects still have reference count 1, so neither gets dropped. This is a **memory leak** - the memory is never freed, but it's still "memory safe" because there are no dangling pointers or undefined behavior.
//
// ## Key Takeaways
//
// 1. **Reference cycles prevent cleanup:** When objects reference each other in a cycle, their reference counts never reach zero
// 2. **Stack overflow vs memory leak:** The stack overflow happens during printing (infinite recursion), the memory leak happens because the cycle prevents deallocation
// 3. **Memory safety:** Even with leaks, Rust maintains memory safety - no crashes, no undefined behavior
// 4. **Solution:** Use `Weak<T>` references to break cycles (covered in the next part of the Rust book)
