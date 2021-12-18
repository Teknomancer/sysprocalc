[![Build](https://github.com/Teknomancer/sysprocalc/workflows/build/badge.svg)](https://github.com/Teknomancer/sysprocalc/actions?query=workflow%3ABuild)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](https://opensource.org/licenses/MIT)
  
# sysprocalc
sysprocalc (system programmer's calculator) is an interactive, command-line expression evaluator for Windows, macOS, Linux. sysprocalc is written entirely in Rust and is the successor to my old C project [nopf](https://github.com/Teknomancer/nopf).

> :warning: **Warning** This is my first Rust project and I'm still learning idiomatic Rust. Inefficient or atypical Rust code is to be expected.

Basic expression parsing and evaluation works. There's also decent test coverage using GitHub continuous integration for Windows, macOS and Linux. This helps identify regressions while modifying core functionality.

Variables/constants and x86-register descriptions aren't supported yet but are planned to be implemented in the future.

### Executable and Library

The project is split into a main executable `sysprocalc` and the core parser/evaluator library (`spceval`). While I don't have any plans of publishing the library as a crate, the library and executable are not tightly coupled. The library exists in its own workspace, to make it easy to publish as a crate in the future.

When the project reaches a mature state, binary downloads may be made available. Currently, to use sysprocalc, you will have to build it from source.

### Building from source

1. Download and [install Rust](https://www.rust-lang.org/tools/install).
2. Clone sysprocalc's git repository to your computer using:
   ```
   git clone https://github.com/Teknomancer/sysprocalc.git
   ```
3. Build, test and run (debug target) using:
   ```
   cargo build && cargo clippy && cargo test --all && cargo run
   ```
   The `--all` is required to run tests for the library and not just the executable.
   
   To build a release target, append `--release` to each of the above `cargo` commands.

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
