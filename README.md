[![Build](https://github.com/Teknomancer/sysprocalc/workflows/build/badge.svg)](https://github.com/Teknomancer/sysprocalc/actions?query=workflow%3ABuild)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](https://opensource.org/licenses/MIT)
  
# sysprocalc
sysprocalc (system programmer's calculator) is an interactive, command-line expression evaluator for Windows, macOS, Linux (or any platform with a Rust compiler and required dependencies).

sysprocalc is the successor to my old C project [nopf](https://github.com/Teknomancer/nopf). sysprocalc is written from scratch using Rust. This is my first Rust project and I'm learning idiomatic Rust but I'm hoping to turn it into a useful application in the near future.

Basic expression parsing and evaluation already works. There's also decent test coverage with GitHub continuous integration (build and testing) for Windows, Linux and macOS which helps greatly while modifying core code.

Variables/constants and common x86-register descriptions aren't supported yet but will be implemented in the near future.

### Executable and Library

The project is split into a main executable `sysprocalc` and the core parser/evaluator library (`spceval`).

Though I do not have any current plans of publishing the library as a crate, the library and executable are not tightly coupled. The library exists in its own workspace, making publishing it as a crate easier in the future.

### Building from source

1. Download and [install Rust](https://www.rust-lang.org/tools/install)
2. Clone sysprocalc's git repository to your computer using:
   ```
   git clone https://github.com/Teknomancer/sysprocalc.git
   ```
3. Build, test and run (debug target) using:
   ```
   cargo build && cargo clippy && cargo test --all && cargo run
   ```
   The `--all` is required to make sure tests in the library are also run and not just the executable's tests.
   
   To build a release target, append `--release` to each of the above commands.

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

Test change to see if workflow works.
