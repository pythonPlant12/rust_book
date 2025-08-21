use core::slice;

fn main() {
    // The unsafe rust's purpose is for 5 superpowers:
    // - Dereference a raw pointer
    // - Call an unsafe function or method
    // - Access or modify a mutable static variable
    // - Implement an unsafe trait
    // - Access fields of a union
    // Keep `unsafe` block small; you'll be thankful later when you investigate
    // memory bugs
    // To isolate unsfae code as much as possible, it's best to enclose such code
    // withint a safe abstraction and provide a safe API, which we'l discuss later in the chapter
    // when we examine unsafe functions and methods. Parts of the standart library are implemented
    // as safe abstractions over unsafe code that hass been audited. Wrapping unsfae code in a safe
    // abstraction prevents uses of `unsafe` from leaking out into all the places that you or your
    // users might want to use the functionality implemented with `unsafe` code, because using a
    // safe abstraction is safe.
    //
    // Le'ts look at each of the five unsafe superpowers in turn. We'll also look at some
    // abtractions that provide a safe interface to unsafe code.
    //
    // ====================================
    // Dereferencing a Raw Pointer
    // ====================================
    //
    // In Rust, compiler ensures that the references are always valid. Unsafe Rust has two new
    // types `Raw pointer` that are similar to references. `Raw pointers` can be immutable or
    // mutable and are written as *const T and *mut T, respectively. The asterist isn't the
    // dereference operator; it's part of the type name. In `raw pointers`, immutable means that
    // the pointer can't be directly assigned to after being dereferenced.
    //
    // Different from references and smart pointers, raw pointers:
    // - Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or
    // multiple mutable pointers to the same location.
    // - Aren't guaranteed to point to valid memory.
    // - Are allowed to be null
    // - Don't implement any automatic cleanup
    //
    // By opting out of having Rust enforce these guarantees, you can give up guaranteed safety in
    // exchange for greater performance or the ability to interface with another language or
    // hardware where Rust's guarantees don't apply.
    //
    // Let's create an immutable and a mutable raw pointer
    let mut num = 5;
    let r1 = &raw const num;
    let r2 = &raw mut num;
    //
    // Notice that we don't include the `unsafe` keyword in this code. We can create raw pointers
    // in safe code; we just can't dereference raw pointers outside an unsafe block, as we'll see
    // in a bit.
    //
    // Take into consideration that here we are pointing to some memory allocation, which can have
    // data or not.
    // let address = 0x012345usize;
    // let r = address as *const i32;
    unsafe {
        println!("r1 is : {}", *r1);
        println!("r2 is : {}", *r2);
    };

    // Unsafe functions
    unsafe fn dangerous() {}
    // You can't call this function outside unsafe {}
    // dangerous(); // ERROR

    unsafe {
        dangerous();
    }
    // Just because a function contains unsafe code doesn't mean we need to mark the entire
    // function as unsafe. IN fact, wrapping unsafe cod ein a safe function is a common
    // abstraction.
    //
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // For example this function which is from Rust, should be implement unsafe code
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        assert!(mid <= len);
        (&mut values[..mid], &mut values[mid..])
        // This will not work as we cannot borrow mut varialbes more than once.
        // Rust’s borrow checker can’t understand that we’re borrowing different parts of the slice; it only knows that we’re borrowing from the same slice twice. Borrowing different parts of a slice is fundamentally okay because the two slices aren’t overlapping, but Rust isn’t smart enough to know this. When we know code is okay, but Rust doesn’t, it’s time to reach for unsafe code.
    }

    fn split_at_mut1(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
    //
    // Most of the chapter if read from the book.
    // 
    // RECOMMENDED: You can use -- Miri -- tool that checks for undefined behavior on runtime.
    // (Very useful for unsafe code). It is an official Rust tool. 
    //
    // Using Miri requires a nightly build of Rust. You can install both a nightly version 
    // of Rust and the Miri tool by typing:
    // `rustup +nightly component add miri`
    //
    // To run Miri on a project type:
    // `cargo +nightly miri run` or `cargo +nightly miri test`
}
