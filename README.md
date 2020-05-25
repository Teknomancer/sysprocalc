# sysprocalc
System Programmer's Calculator Expression Evaluator

*Project work-in-progress.*

Rewrite of [https://github.com/Teknomancer/nopf]nopf written using Rust.

This is my first time using Rust, so don't expect 100% idiomatic Rust. This is mainly to learn Rust and also to get something useful out of it.

Build using
```
cargo build && cargo clippy && cargo test --all
```

The "--all" is required, otherwise only the main executable's tests will run and not the tests for the library.
