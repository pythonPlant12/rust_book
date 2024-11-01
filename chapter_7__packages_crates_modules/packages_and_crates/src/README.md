## Managing Growing Projects with Packages, Crates, and Modules

##### A package can contain multiple binary crates and optionaly one library crate.
##### As a package grows, you can extract parts into separate crates that become external dependencies.
###### In this chapter, we'll cover all this techniques.
##### For a very large projects, cargo provides workspaces, those will be covered in Chapter 14.
##### Module system includes:
- **Packages**: A Cargo feature that lets you build, test, and share crates.
- **Crates**: A tree of modules that produces a library or executable.
- **Modules and use**: Let you control the organization, scope, and privacy of paths.
- **Paths**: A way of naming an item, such as a struct, function, or module.
