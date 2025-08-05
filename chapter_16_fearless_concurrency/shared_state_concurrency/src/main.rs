use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Mutex in a single-thread example
    // let m = Mutex::new(5);
    //
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }
    //
    // println!("m = {m:?}");
    // println!("m.lock().unwrap(): {}", m.lock().unwrap()); // This by the way will print the i32
    //
    // Mutex with multiple threads
    // ---------------------------------------------------
    // let counter = Mutex::new(0);
    // let mut handles = vec![];
    //
    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    //
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("Result: {}", *counter.lock().unwrap());
    // In this example we'll get an error because we pass the counter value to multiple threads at
    // the same time.
    // ---------------------------------------------------
    // Let's make it better
    // In Chapter 15, we gave a value to multiple owners by using the smart pointer `Rc<T>` to
    // create a reference counted value. Let's do the same here and see what happens. We'll wrap
    // the `Mutex<T>` in `Rc<T>` and clone the `Rc<T>` before moving ownership to the thread.
    //
    // However, here we'll get an error again. The Trait `Send` is not implemented for Rc<Mutex<i32>>.
    // This is one of the traits which insures the types we use with threads are meant for use in
    // concurrent situations.
    //
    // Another error that we get is: `Rc<Mutex<i32>>` cannot be sent between threads safely.
    //
    // Unfortunately, `Rc<T>` is not safe to share across threads. When Rc<T> manages the reference
    // count, it adds to the count for each call to `clone` and substracts from the count when each
    // clone is dropped. But it doesn't use any concurrency primitives to make sure that changes to
    // the count can't be interrupted by another thread. This could lead to wrong counts - subtle
    // bugs that could in turn lead to memory leaks or a value being dropped before we're done with
    // it. What we need is a type that is exactly like `Rc<T>` but one that makes changes to the
    // reference count in a thread-safe way.
    //
    // Fortunately, `Arc<T>` is a type like `Rc<T>` that is safe to use in concurrent situations.
    // The `a` stands for atomic, meaning it's an 'atomically reference-counted' type. Atomics are
    // an additional kind of concurrency primitive that we won't cover in detail here: see the
    // standard library documentation for `std::sync::atomic` for more details. At this point, you
    // just need to know that atomics work like primitive types but are safe to share across
    // threads.
    //
    // You might then wonder why all primitive types aren't atomic and why standard library types
    // aren't implemented to use `Arc<T>` by default. The reason is that thread safety comes with a
    // performance penalty that you only want to pay when you really need to. If you're just
    // performing operations on values within a single thread, your code can run faster if it
    // doesn't have to enforce the guarantees atomics provide.
    //
    // Let's return to our example: `Arc<T>` and `Rc<T>` have the same API, so we fix our program
    // by changing the `use` line, the call to `new`, and the call to `clone`.
    //
    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];
    //
    // for _ in 0..10 {
    //     let counter = Rc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //
    //         *num +=1;
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("Result: {}", *counter.lock().unwrap());
    //
    // // Correct code that will compile

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for thread_num in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
            println!("Thread number {thread_num}: *num value: {num}");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // This will be the result: 
    // Thread number 1: *num value: 1
    // Thread number 2: *num value: 2
    // Thread number 0: *num value: 3
    // Thread number 3: *num value: 4
    // Thread number 4: *num value: 5
    // Thread number 5: *num value: 6
    // Thread number 6: *num value: 7
    // Thread number 7: *num value: 8
    // Thread number 8: *num value: 9
    // Thread number 9: *num value: 10
    // Result: 10
    //
    // ----------------------------------
    //
    // As with many types, we create a `Mutex<T>` using the associated function `new`. To access
    // the data inside the mutex, we use the `lock()` method to acquire the lock.
    // This call will block the current thread so it can't do any work until it's our turn to have
    // the lock.
    //
    // The call to `lock` would fail if another thread holding the lock panicked. In that case, no
    // one would ever be able to get the lock, so we've chosen to `unwrap` and have this thread
    // panic if we're in that situation.
    //
    // After we've acquired the lock, we can treat the return value, named `num` in this case, as a
    // mutable reference to the data inside. The type system ensures that we acquire a lock before
    // using the value m. The type of `m` is `Mutex<i32>` not <i32>, so we must call lock to be
    // able to use the i32 value. We can't forget; the type system won't let us access the inner
    // i32 otherwise.
    //
    // As you might suspect, `Mutex<T>` is a smart pointer. More accurately, the call to `lock`
    // returns a smart pointer called `MutexGuard`, wrapped in a `LockResult` that we handled with
    // the call to `unwrap`. The `MutexGuard` smart pointer implements `Deref` to point at our
    // inner data; the smart pointer also has a `Drop` implementation that releases the lock
    // automatically when a `MutexGuard` goes out of scope, which happens at the end of the inner
    // scope. As a result, we don't risk forgetting to release the lock and blocking the mutex from
    // being used by other threads, because the lock release happens automatically.
}

// Message passing is a fine way to handle concurrency, but it's not the only way. Another method
// would be for multiple threads to access the same shared data. Consider this part of the slogan
// from the Go language documentation again: "Do not communicate by sharing memory".
//
// What whould communicating by sharing memory look like? In addition, why would message-passing
// enthusiasts caution not to use memory sharing?
//
// In a way, channels in any programming language are similar to single ownership, because once you
// transfer a value down a channel, you should no longer use that value. Shared-memory concurrency
// is like multiple ownership: multiple threads can access the same memory location at the same
// time.
// As you saw in Chapter 15, were smart pointers made multiple ownership possible, multiple
// ownership can add complexity because these different owners need managing. Rust's type system
// and ownership rules greatly assist in getting this management correct. For an example, let's
// look at mutexes, one of the more common concurrency primitives for shared memory.
//
// =================================================
// Using Mutexes to Allow Access to Data from One Thread at a Time
// =================================================
// `Mutex` is an abbreviation for `mutual exclusion`, as in a mutex allows only one thread to
// access some data at any given time. To access the data in a mutex, a thread must first signal
// that it wants access by asking to acquire the `mutex's lock`. The lock is a data structure that
// is part of the mutex that keeps track of who currently has exclusive access to the data.
// Therefore, the mutex is described as "guarding" the data it holds via the locking system.
//
// Mutexes have a reputation for being difficult to use because you have to remember two rules:
// - You must attempt to acquire the lock before using the data.
// - When you're done with the data that the mutex guards, you must unlock the data so other
// threads can acquire the lock.
//
// For a real-world metaphor for a mutex, imagine a panel discussion at a conference with only one
// microphone. Before a penlist can speak, they have to ask or signal that they want to use the
// microphone. When they get the microphone, they can talk for as long as they want to and then
// hand the microphone to the next panelist who requests to speak. If a panelist forgets to hand
// the microphone off when they're finished with it, no one else is able to speak. If management of
// the shared microphone goes wrong, the panel won't work as planned!
//
// Management of mutexes can be incredibly tricky to get right, which is why so many people are
// enthusiastic about channels. However, thanks to Rust's type system and ownership rules, you
// can't get locking and unlocking wrong.
//
// ================================================
// The API of Mutex<T>
// ================================================
// As an example of how to use a mutex, let's start by using a mutex in a single-threaded context,
// as shown below (main):
//
//
// ================================================
// Sharing a `Mutex<T>` Between Multiple Threads
// ================================================
// Now let's try to share a value between multiple threads using `Mutex<T>`. We'll spin up 10
// threads and have them each increment a counter value by 1. so the counter goes from 0 to 10. The
// example below will have a compiler error, and we'll use that error to learn more about using
// `Mutex<T>` and how Rust helps us use it correctly.
