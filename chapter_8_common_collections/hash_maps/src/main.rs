use std::{collections::HashMap};

fn main() {
    let mut new_hashmap: HashMap<&str, &str> = HashMap::new();
    // Using str will not take the ownership as the value is copied as it is in stack memory
    let str_in_stack_for_key = "key";
    let str_in_stack_for_value = "value";
    let str_in_heap_for_key = String::from("hi");
    let str_in_heap_for_value = String::from("hello_heap");

    new_hashmap.insert("hi", "hi");
    new_hashmap.insert(str_in_stack_for_key, str_in_stack_for_value);
    new_hashmap.insert("hi", "hello");
    // This won't work because of the borrow checker
    // new_hashmap.insert(&str_in_heap_for_key, &mut str_in_heap_for_value);
    // str_in_heap_for_value.push_str(" pushed string");

    new_hashmap.insert(&str_in_heap_for_key, &str_in_heap_for_value);
    let formatted = format!("{str_in_stack_for_key}, {str_in_stack_for_value}");
    println!("{formatted}");
    let first_key = new_hashmap.get("hi").copied().unwrap_or("not found");
    println!("{first_key}");

    // To check if the specific key exist, we can use .entry(), this returns an Enum Entry
    // This will not be added as "hi" key already exists
    new_hashmap.entry("hi").or_insert("didn't exist");
    // This will be added as key "hello1" doesn't exist in the HashMap
    new_hashmap.entry("hello1").or_insert("didn't exist");

    // Updating the value based on the old value
    let mut map = HashMap::new();
    let text = "hello world wonderful world";
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{new_hashmap:#?}");
    println!("{map:#?}")
}
