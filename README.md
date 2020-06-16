[![Build](https://github.com/Teknomancer/sysprocalc/workflows/build/badge.svg)](https://github.com/Teknomancer/sysprocalc/actions?query=workflow%3ABuild)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](https://opensource.org/licenses/MIT)
  
# sysprocalc
sysprocalc (system programmer's calculator) is an interactive, command-line expression evaluator.

This is a my old C project [nopf](https://github.com/Teknomancer/nopf) re-written using Rust. This is also my first Rust project and I'm learning to implement idiomatic Rust concepts. This was started to learn Rust but I'm hoping to turn into a more full-fledged application in the near future.

Basic expression parsing and evaluation already works and has a decent test coverage with continuous integration on Windows and Linux hosts.

### Executable and Library

The project is split into a main executable `sysprocalc` and the core parser/evaluator library (`spceval`).

Though I do not have any current plans of publishing the library as a crate, the library and executable are not tightly coupled. The library exists in its own workspace, making publishing as a crate easier in the future.

### Building from source

Build, test and run using:
```
cargo build && cargo clippy && cargo test --all && cargo run
```

The `--all` is required to make sure tests in the library are also run and not just the executable's tests.

### License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
