# Customizing Build with Release Profiles
- `cargo build` - for development (optimized for development)
- `cargo build --release` - for release project

The `dev` and `release` are different profiles used by compiler.

In `Cargo.toml` you can specify different settings:

DEV
````
```
[profile.dev]
opt-level = 0
```
RELEASE
```
[profile.release]
opt-level = 3 // 3 is maximum
```
