fn main() {
    // A crate is the smalles amout of code that Rust compiler considers at a time.
    // You can rust a crate (one file) with rustc which compiler will consider as a crate,
    // in this crate there can be multiple modules.
    // A crate can come in one of two forms. Binary crate or library crate
    // Binary crate:
    // Are programs you can compile to an executable that you can run, such as command line
    // programm or a server.
    // Each should have fn main() {}


    // Library crates:
    // They don't have fn main() {} and they don't compile to an executable. They define the
    // functionality intented to be shared with multiple projects.
    // Most of the time when you hear crate, it refers to library crate. Equivalent to library.

    // A package is a bundle of one or more crates that provides a set of functionality.
    // A package contain Cargo.toml file that describes how to build those crates.
    // Cargo is actually a package that contains the binary crate for the command-line tool you've
    // been using to build your code. The Cargo also contains a library crate that the binary crate depends on.

    // A package can contain as many binary crates as you like, but at most only one liobrary crate.
    // A package must contain at least one crate, whether that's library or binary.

    // When we ruyn cargo new my-project there's a Cargo.toml file, giving as a package
    // There's also a src directory that contains main.rs
    // However, in Cargo.toml there's no mention to src/main.rs.
    // Cargo follows a convention that src/main.rs is the crate root of binary crate with the same name as the package.
    // Likewise, Cargo knows that if the package directory contains src/lib.rs, the packages contains a library
    // crate with the same name as the package, adn src/lib.rs is its crate root.
    // Cargo passes the crate root files to rustc to build the library or binary.

    // In our simple example, we have a package that only contains src/main.rs, meaning it only contains
    // a binary crate named my-project. If a package contains src/main.rs and src/lib.rs, it has two crates:
    // a binary and a library, both with the same name as the package. A package can have multiple binary crates
    // by placing files in the src/bin directory: each file will be a separate binary crate.
}
