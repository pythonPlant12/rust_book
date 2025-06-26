use crate::garden::vegetables::test as test_vegetables;
pub mod vegetables;
pub mod vegetables1;
pub fn test() {
    println!("Called from src/garden/mod.rs (Module)");
    test_vegetables();
}