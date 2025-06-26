pub fn test() {
    // You cannot have the module/submodule in:
    // src/garden/vegetables/mod.rs
    // src/garden/vegetables.rs
    // You'll get an error that on of them should be renamed for the ambiguity
    // println!("Called from src/garden/vegetables.rs (Submodule)");
    println!("Called from src/garden/vegetables1.rs (Submodule)");
}