// use std::{sync::{mpsc::channel, Arc, Mutex}, thread, time::Duration};
//
// fn main() {
//     let stack_value_from_main = 10;
//     let heap_value_from_main = Arc::new(Mutex::new(String::from("Hello")));
//     let (tx, rx) = channel();
//     let _ = tx.send(Arc::clone(&heap_value_from_main));
//
//     let thread1 = thread::spawn(move || {
//         let receive_from_channel = rx.try_recv().unwrap();
//         let mut data_from_channel = receive_from_channel.lock().unwrap();
//         thread::sleep(Duration::from_secs(2));
//         data_from_channel.push_str(" world!");
//         println!("Printing value from heap in THREAD: {data_from_channel}");
//         println!("Printing value from stack in THREAD: {stack_value_from_main}");
//     });
//
//     // This will not work as this value is moved (because it was initialized on the heap)
//     println!("Printing value from heap in MAIN: ");
//     let mut value = heap_value_from_main.lock().unwrap();
//     value.push_str(" from main!");
//     thread1.join().unwrap();
//
//     println!("Printing value from heap in MAIN: {value}");
//     println!("Printing value from stack in MAIN: {stack_value_from_main}");
// }
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let string = Arc::new(Mutex::new(String::from("Hello")));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&string);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            num.push_str(" world!");
        });
        handles.push(handle);
    }

    {
        let borrowed_string = string.lock().unwrap();
        println!("Borrowed string from main: {borrowed_string}");
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *string.lock().unwrap());
}
