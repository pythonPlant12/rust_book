use std::error::Error;
use std::{env, fs, process};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // Instead of using new we'll use build, as many programmers don't expect the ::new() function
    // to fail
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let search_string = args.next().unwrap_or_else(|| {
            eprintln!("Search string is not defined in params");
            process::exit(1)
        });

        let file_path = args.next().unwrap_or_else(|| {
            eprintln!("File path is not defined in params");
            process::exit(1)
        });

        let ignore_camel_case = match args.next() {
            Some(_) => true,
            None => env::var("IGNORE_CASE").is_ok(),
        };

        Ok(Config {
            query: search_string,
            file_path,
            ignore_case: ignore_camel_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // <Box<dyn Error>> is a trait object, we will cover it later in Chapter 18.
    // This means: it's is dyn (dynamic) object which implements the trait Error
    // This gives us flexibility to return different types in different error cases.
    let content = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    for line in &results {
        println!("{line}");
    }
    Ok(())
    // This Ok(()) syntax, is a bit strange, but using () like this is the idiomatic way to
    // indicate that we're calling run for its side effects only; it doesn'5t return a value we
    // need
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();


    contents.lines().filter(|line| {
        line.contains(query)
    }).collect()
    // for line in contents.lines() {
    //     // Lines returns an iterator, we'll talk about iterators in
    //     // Chapter 13
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // let mut results = Vec::new();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
