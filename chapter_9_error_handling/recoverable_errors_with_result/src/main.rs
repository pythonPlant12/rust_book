use std::fs::{read, File};
use std::io::{Error, ErrorKind, Read};

fn main() {
    // Rust doesn't have try, except.
    // For this scenarios you should use Result<T, E>.
    // We use generics here, so we can use this Result in many scenarios where the result or error
    // types may differ
    // This is already implemented by rust, and this is the enum which returns File::open function
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E)
    // }
    // Let call the function that will return the Result value as the function may fail
    let greeting_file_result = File::open("Cargo1.lock");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file {error}")
    // };
    // println!("{greeting_file:#?}");

    // Right now, the code will panic!, no matter why File::open failed.
    // However, we want to take different actions for different failure reasons.
    match greeting_file_result { 
        Ok(file) => file, // If file exists, Ok
        Err(error) => match error.kind() { // If file doesn't exist and get error, we match with
            // kind of error, in our case only checking ErrorKind::NotFound and the rest 
            ErrorKind::NotFound => match File::create("hello.txt") { // In case the file is not
                // found, we create it, again checking if it was created succesfully.
                Ok(created_file) => created_file,
                Err(error) => panic!("Problem creating the file {error}")
            },
            _ => panic!("Problem opening the file {error:?}")
        }
    };


    fn open_and_read_file() -> Result<String, Error> {
        let mut string_from_file: String = String::new();
        File::open("Cargo.lock")?.read_to_string(&mut string_from_file)?;
        Ok(string_from_file)
    }
}
