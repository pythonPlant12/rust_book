// First we want our program to receive two arguments, rather for cargo, it's for our project
// 'cargo run -- searchstring example-filename.txt'
// To do so, we can use some library, but in our case, we'll do it ourselves
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args = env::args();
    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
