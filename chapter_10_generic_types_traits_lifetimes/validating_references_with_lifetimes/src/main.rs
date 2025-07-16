fn main() {
    // Lifetimes are another kind of generic.
    // Rather than ensuring that a type has the behavior we want, lifetimes ensure that references
    // are valid as long as we need them to be.
    // Every reference in Rust has its lifetime
    // Normally, we don't need to specify a lifetime for references as compiler is doing it for us,
    // however, sometimes we'll need to do it manually.

    // Lets see some examples
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {r}")
    // This will not work, as we reference a variable that is going out of scope when we use it.
    // In this concrete example this called dangling references

}
