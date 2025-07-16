pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist1");
        }
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn server_order() {}
        fn take_payment() {}
    }
}
// Here as you can see I can use modules the same way but even in the same file, 
// they should also be public in order to be able to be used
use crate::front_of_house::hosting::add_to_waitlist as add_to_waitlist_full_imported;
use crate::front_of_house as other_front_of_house;
fn main() {
    add_to_waitlist_full_imported();
    other_front_of_house::hosting::add_to_waitlist()
}
