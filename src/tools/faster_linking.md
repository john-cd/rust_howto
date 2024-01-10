# Faster linking

The Rust compiler spends a lot of time in the "link" step. LLD is much faster at linking than the default Rust linker.

The default linker does a good job, but there are faster alternatives depending on the operating
system you are using:

- `lld` on Windows and Linux, a linker developed by the LLVM project;
- `zld` on MacOS.

To speed up the linking phase you have to install the alternative linker on your machine and add
this configuration file to the project:

```toml
# .cargo/config.toml
# On Windows
# ```
# cargo install -f cargo-binutils
# rustup component add llvm-tools-preview
# ```
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# On Linux:
# - Ubuntu, `sudo apt-get install lld clang`
# - Arch, `sudo pacman -S lld clang`
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]

# On MacOS, `brew install michaeleisel/zld/zld`
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
```

`cargo-binutils` packages Cargo subcommands to invoke the LLVM tools shipped with the Rust toolchain.

## Alternative - Mold linker

`mold` is up to 5× faster than `lld`, but with a few caveats like limited platform support and occasional stability issues. To install mold, run `sudo apt-get install mold clang` in Ubuntu.

You will also need to add the following to your Cargo config at `.cargo/config.toml`:

```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=/usr/bin/mold"]
```

## Reference

[Enable Fast Compiles (Bevy)][enable-fast-compiles]⮳

{{#include ../refs/link-refs.md}}
