use std::{sync::mpsc, thread, time::Duration};

fn main() {
    // One increasignly popular approach to ensuring safe concurrency is MESSAGE PASSING, where
    // threads or actors communicate by sending each other messages containing data. Here's the
    // idea in a slogan from the 'GO Language Documentation': "Do not communicate by sharing
    // memory; instead, share memory by communicating".
    //
    // To accomplish message-sending concurrency, Rust's standard library provides an
    // implementation of CHANNELS. A channel is a general programming concept by which data is sent
    // from one thread to another.
    //
    // You can imagine a channel in programming as being like a directional channel of water, such
    // as a stream of a river. If you put something like a rubber duck into a river, it will travel
    // downstream to the end of the waterway.
    //
    // A channel has two halves: a transmitter and a receiver. The transmitter half is the upstream
    // location where you put the rubber duck into the river, and the receiver half is where the
    // rubber duck ends up downstream. One part of your code calls methods on the transmitter with
    // the data you want to send, and another part checks the receiving end for arriving messages.
    // A channel is said to be closed if either the transmitter or receiver half is dropped.
    //
    // Here, we'll work up to a program that has one thread to generate values and send them down a
    // channel, and another thread that will receive the values and print them out. We'll be
    // sending simple values between threads using a channel to illustrate the feature. Once you're
    // familiar with the technique, you could use channels for any threads that need to communicate
    // with each other, such as a chat system or a system where many threads perform parts of a
    // calculation and send the parts to one thread that aggregates the results.
    //
    // Let's create a channel but not do anything with it. Note that this won't compile yet because
    // Rust can't tell what type of values we want to send over the channel
    //
    // use std::sync::mpsc; (mpsc -> multiple producer, single customer)
    // In short, the way Rust's standard library implements channels means a channel can have
    // multiple sending ends that produce values but only one receiving end that consumes those
    // values. Imagine multikple streams flowing together into one big river: everything sent down
    // any of the streams will end up in one river at the end. We'll start with a single producer
    // for now, but we'll add multiple producers when we get this example working.
    //
    // The `mpsc::channel` function returns a tuple, the first element of which is the sending end
    // (the transmitter) and the second element of which is the receiving end (the receiver). The
    // abbreviations `tx` and `rx` are traditionally used in many fields for transmitter and
    // receiver, respectively, so we name our variables as such to indicate each end. We're using a
    // `let` statement with a pattern that destructures the tuples; we'll discuss the use of
    // patterns in `let` statements and destructuring in Chapter 19.
    // For now, know that using a `let` statement this way is a convenient approach to extract the
    // pieces of the tuple returned by `mpsc::channel`
    //
    // Let's move the transmitting end into a spawned thread and have it send one string so the
    // spawned thread is communicating with the main thread. This is like putting a rubber duck in
    // the river upstream or sending a chat message from one thread to another.
    //
    // let (tx, rx) = mpsc::channel();
    //
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // We need to move the value of tx, so the spawned thread owns `tx`. As
        // it is necessary to send messages through the channel.
        let val = String::from("hi");
        tx.send(val).unwrap(); // The .send() function takes the ownership of its parameter and
        // when the value is moved, the receiver takes the ownership of it.
        // println!("val is {val}"); This will not work as the value was moved into the main thread
        // by sending message via transmitter
    });
    // The transmitter has a `send` method that takes the value we want to send. The `send` method
    // returns a <Result<T, E>> type, so if the receiver has already been dropped and there's
    // nowhere to send a value, the send operation will return an error. In this example, we're
    // calling unwrap to panic in case of an error. But in a real application, we would handle it
    // properly: return to Chapter 9 to review strategies for proper error handling.
    //
    // Now let's try to retrieve the message in the main thread.
    let received = rx.recv().unwrap();
    println!("Got: {received}");

    // The receiver has two useful methods: `recv` and `try_recv`. We're using `recv`, short for
    // recieve, which will block the main thread's execution and wait until a value is sent down
    // the channel. Once a value is sent, `recv` will return it in a `Result<T, E>`. When the
    // transmitter closes, `recv` will return an error to signal that no more values will be
    // coming.
    //
    // The `try_recv` method doesn't block, but will instead return a `Result<T, E>` immediately:
    // an `Ok` value holding a message if one is available and an `Err` value if there aren't any
    // messages this time. Using `try_recv` is useful if this thread has other work to do while
    // waiting for messages: we could write a loop that calls `try_recv` every so often, handles a
    // message if one is available, and otherwise does other work for a little while until checking
    // again.
    //
    // We've used `recv` in this example for simplicity: we don't have any other work for the main
    // thread to do other that wait for messages, so blocking the main thread is appropiate.
    //
    //
    // =========================================================
    // Channels and Ownership Transference
    // =========================================================
    // The ownership rules play a vital role in message sending because they help you write safe,
    // concurrent code. Preventing errors in concurrent programming is the advantage of thinking
    // about ownership throughout your Rust programs. Let's do an experiment to show how channels
    // and ownership work together to prevent problems: we'll try to use a `val` in the spawned
    // thread after we've sent it down the channel.
    // The next code won't compile:
    //
    //
    // =========================================================
    // Sending Multiple Values and Seeing the Receiver Waiting
    // =========================================================
    let (tx1, rx1) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            println!("Printing val from second thread: {val}");
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx1 {
        println!("Printing val from main thread: {received}");
    }
    println!("Main thread finished");
    // This time, the spawned thread has a vector of string that we want to send to the main
    // thread. We iterate over them, sending each individually, and pause between each by calling
    // the `thread::sleep` function with a `Duration` value of one second.
    //
    // In the main thread, we're not calling the `recv` function explicitly anymore: instead,
    // we're treating `rx` as an iterator. For each value received, we're printing it. When the
    // channel is closed, iteration will end.
    //
    // Because we don't have any code that pauses or delays in the `for` loop in the main thread,
    // we can tell that the main thread is waiting to receive values from the spawned thread.
    //
    // ===============================================
    // Creating Multiple Producers by Cloning the Transmitter
    // ===============================================
    let (tx2, rx2) = mpsc::channel();

    let tx3 = tx2.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("third thread hi"),
            String::from("third thread from"),
            String::from("third thread the"),
            String::from("third thread thread"),
        ];

        for val in vals {
            println!("Printing val from third thread: {val}");
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("fourth thread hi"),
            String::from("fourth thread from"),
            String::from("fourth thread the"),
            String::from("fourth thread thread"),
        ];

        for val in vals {
            println!("Printing val from third thread: {val}");
            tx3.send(val).unwrap();
            thread::sleep(Duration::from_millis(55));
        }
    });
    // Take into consideration thatn this loop blocks the main thread until ALL transmitters are
    // closed. 
    // The ireatoror `rx2` will continue until the channel is completely closed. The channel only
    // closes when ALL transmitters are dropped.
    for val in rx2 {
        println!("Printed from multiple transmitters (tx2, tx3 (<tx2.clone>): {val}");
    }
    println!("Finishing the main thread");

    // This time, before we create the first spawned thread, we call `clone` on the transmitter.
    // This will give us a new transmitter we can pass to the first spawned thread. We pass the
    // original transmitter to a second spawned thread. This gives us two threads, each sending
    // different messages to the one receiver.
    //
    // You might see the values in another order, depending on your system. This is what makes
    // concurrency interesting as well as difficult. If you experiment with `thread::sleep`, giving
    // it various values in the different threads, each run will be more nondeterministic and
    // create a different output each time. 
}
