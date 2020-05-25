# sysprocalc
System Programmer's Calculator Expression Evaluator

Rewrite of [https://github.com/Teknomancer/nopf]nopf written using Rust.

Build using
```
cargo build && cargo clippy && cargo test --all
```

The "--all" is required, otherwise only the main executable's tests will run and not the tests for the library.
