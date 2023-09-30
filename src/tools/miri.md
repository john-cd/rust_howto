# Miri Interpreter

[Miri]( https://github.com/rust-lang/miri )

An experimental interpreter for Rust's mid-level intermediate representation (MIR). It can run binaries and test suites of cargo projects and detect certain classes of undefined behavior. It can also perform cross-interpretation for arbitrary foreign targets.

```bash
rustup +nightly component add miri
cargo clean
cargo miri test
# or
cargo miri run
```
