# Installing Binaries with `cargo install`
The cargo install command allows you to install and use binary crates locally.
To be binary crate it should have a binary file src/main.rs or another file specified as binary, as opposed to 
the library crate that isn't runnable on its own but is suitable for including withit other programs.

Usually, crates have information in the README file about whether a crate is a library, has a binary target, or both.
All binaries are stored in the installation root's bin folder. 
