# Rustup

`rustup` is a toolchain multiplexer. It installs and manages many Rust toolchains and presents them all through a single set of tools installed to `~/.cargo/bin`. The `rustc` and `cargo` executables installed e.g. in `~/.cargo/bin` are proxies that delegate to the real toolchain.

This is similar to Python's `pyenv` or Node's `nvm`.

## Key commands

```sh
rustup help
rustup toolchain help   # Show the help page for a subcommand (like toolchain)

rustup doc              # Open the local documentation in your browser

rustup update           # Update to a new verion of Rust

rustup show             # Show which toolchain will be used in the current directory

rustup target list      # Show which toolchain will be used in the current directory
rustup toolchain list   # Overview of what is installed on your system
rustup component list   # See a list of available and installed components.
```

[Rustup command examples][rustup-command-examples]⮳

## See also

[Rustup documentation][rustup-documentation]⮳

[rustup-command-examples]: https://rust-lang.github.io/rustup/examples.html
[rustup-documentation]: https://rust-lang.github.io/rustup/
{{#include ../links.md}}
