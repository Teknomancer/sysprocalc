[![Build](https://github.com/Teknomancer/sysprocalc/workflows/Build/badge.svg)](https://github.com/Teknomancer/sysprocalc/actions?query=workflow%3ABuild)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
 
  
# sysprocalc
System Programmer's Calculator Expression Evaluator

My old C project [nopf](https://github.com/Teknomancer/nopf) re-written using Rust.

**This is my first time using Rust, don't expect 100% idiomatic Rust. This is mainly to learn Rust while hopefully getting something useful out of it. Work-in-progress. Not fit for general use.**

Build and run using:
```
cargo build && cargo clippy && cargo test --all && cargo run
```

The `--all` is required, otherwise only the main executable's tests will run and not the tests for the `spceval` library.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

