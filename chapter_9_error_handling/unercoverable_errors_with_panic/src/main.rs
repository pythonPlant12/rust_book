fn main() {
    // By default, when a programm start to panic, the programm start to "unwinding".
    // That means that it starts to go to all the past functions and clear all the data it
    // encounters
    // 
    // In case you don't want it, you can add to your Cargo.toml file the following:
    // [profile.release]
    // panic = 'abort'
    //
    // This will avoid clearing memory and all the data, so the memory will need to be cleared by
    // your operating system. 
    // This is useful if you want your binary programm to be as small as possible.
    // panic!("Crashed a programm"); // This will crash the programm
    // 
    // When the programm panic, you can see the backtrace of the execution which caused the error
    // The backtrace is a list of all the functions that has been called to get to the error
    // You can set up the RUST_BACKTRACE = 1 environmental variable to get the backtrace
    let v = vec![1, 2, 3];
    v[99];
}
