use std::thread;
use std::time::Duration;

fn main() {
    // Even creating new threads, if the main thread finished first, the rest of the threads will
    // stop the execution, even they didn't finish what they would do.
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {i} from the spawned thread!");
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {i} from the main thread");
    //     thread::sleep(Duration::from_millis(1));
    // }

    // We can fix the problem of the spawned thread not running or ending prematurely by saving the
    // return value of thread::spawn in a varialbe. The return type of `thread::spawn` is
    // `JoinHandle<T>`. A `JoinHandle<T>` is an owned value that, when we call the `join` method on
    // it, will wait for its thread to finish. Next listing shows how to use the `JoinHandle<T>` of
    // the thread when created and how to call `join` to make sure the spawned thread finished before
    // `main` exits.

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
        }
    });

    // If we add it above, it will not coninue the code until this threads are finished.
    // NOTE: Such ass where join is called, can affect whether or not your threads run at the same
    // time.
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    // If we add this code here, it will wait till threads finish their execution here.
    // handle.join().unwrap();

    // Now the main thread will wait until the spawned threads are finished.
    // 
    //
    //
    // ==========================================
    // Using `move` Closures with Threads
    // ==========================================
    let v = vec![1, 2, 3];
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });
    //
    // By adding `move` keyword, before the closure, we force the closure to take ownership of the
    // values it's using rather than allowing  Rust to infer that it should borrow the values. This
    // modification will allow us to run the code.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // However, again, we will not be able to call drop(v) on the main thread, as the ownership is
    // moved to the second thread. So Rust won't let us do it.

    // drop(v);
    // If rust would allow us to do so, the borrowed value from another thread would be able to be
    // dangling.
    //
    // The closure uses `v`, so it will capture `v` and make it part of the closure's environment.
    // Because `thread::spawn` runs this closure in a new thread, we should be able to access `v`
    // inside that new thread. But when we compiler this example, we get the following error;
    // (`thread may outlive borrowed value `v`).
    //
    // Rust infers how to capture `v`, and because `prinltn!` only needs a reference to `v`, the
    // closure tries to borrow `v`. However, there's a problem: Rust can't tell how long the
    // spawned thread will run, so it doesn't know whether the reference to `v` will always valid.
    handle.join().unwrap();
}
