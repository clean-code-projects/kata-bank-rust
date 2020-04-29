# Bank Kata in Rust

This is a [bank kata](https://katalyst.codurance.com/bank) implemented in [Rust](https://www.rust-lang.org/).

## Contribute

### Build

```sh
# build library and executable
cargo build

# build + run
cargo run

# compile but don't build
cargo check
```

### Test

```sh
# run all tests: unit, integration, doc
cargo test

# run specific integration test file
# (assume `tests/specific_test.rs` is the test file)
cargo test --test specific_test
```

If you like to run test continuously,
consider installing [cargo watch](https://github.com/passcod/cargo-watch)

```sh
cargo watch -x test

# with print out
cargo watch -x "test -- --nocapture"
```

## Development Environment

We will primarily be using VS Code.

Here are some recommended extensions in VSCode used when working on this project:

- Rust Extension Pack
- vscode-rust-syntax
- EditorConfig for VS Code
- markdownlint
