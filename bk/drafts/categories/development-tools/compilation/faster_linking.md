# Optimize Linking {#faster-linking}

{{#include faster_linking.incl.md}}

[![cat~compilers][cat~compilers~badge]][cat~compilers]{{hi:Compilers}}

Optimizing Rust linking involves several strategies to reduce binary size and link time.

- ThinLTO: A variant of LTO that can offer a better balance between compile time and link time.
- Reducing Dependencies: Fewer dependencies mean less code for the linker to process. Analyze your dependencies with [cargo][p~cargo] tree.
- Code Size Reduction: Smaller code size can lead to faster linking. Techniques like minimizing [generics][p~generics] and using more compact data structures can help.
- Linker Flags: Experiment with linker flags, but be careful and measure the impact.
- Profiling: Use profiling tools to identify bottlenecks in the linking process. This is less common than compile-time profiling.

- [[development-tools_profiling | Development Tools Profiling]].

## `rustc` Configuration {#skip}

### Link-Time Optimization (LTO) {#skip}

Enabling LTO allows the compiler to perform optimizations across the entire program during the linking phase. This can significantly reduce code size and improve [performance][p~performance] by eliminating dead code and inlining functions more effectively. Use the `-C lto=fat` or `-C lto=thin` (faster but less aggressive) compiler flags. LTO typically requires more memory and time during compilation.

Link-Time Optimization (LTO) is controlled via `Cargo.toml`. Can sometimes improve linking times, but often increases compile time. Experiment to see if it helps.

## Codegen Units {#skip}

Increasing the number of codegen units (using -C codegen-units=N) can improve parallelism during compilation, potentially reducing compile time. However, this can sometimes hinder LTO effectiveness. Experiment to find the optimal balance.

## Panic Strategy {#skip}

The default panic strategy (unwind) includes unwinding information, which increases binary size. Switching to the abort panic strategy (using `-C panic=abort`) reduces binary size but prevents stack unwinding in case of a panic. Use abort only if unwinding is not required.

## Strip Symbols {#skip}

Stripping debug symbols from the final binary using compiler flags like `-C strip=debuginfo` significantly reduces binary size. This is essential for release builds.

## Minimize Dependencies {#skip}

Reducing the number of dependencies, especially those with large or complex codebases, directly impacts link time and binary size. Analyze dependencies and consider alternatives if possible.

## Static Linking {#skip}

Static linking (using `-C prefer-dynamic=no`) can sometimes reduce binary size if shared libraries introduce overhead. However, it can also increase the size if multiple binaries link against the same library. Consider the trade-offs.

Generally faster than dynamic linking. Often the default in Rust.

## Optimize Dependencies {#skip}

Ensure dependencies are also built with optimizations enabled. This can be achieved by setting appropriate build profiles for dependencies in your `Cargo.toml`.

## Profile-Guided Optimization (PGO) {#skip}

PGO uses runtime profiling data to guide compiler optimizations, potentially leading to better [performance][p~performance] and smaller binaries. This involves a more complex build process but can be beneficial for performance-critical applications.

## Linker Flags {#skip}

Using linker-specific flags (e.g., -Wl,--gc-sections for GCC/ld) can help remove unused code and data sections, further reducing binary size.

## Incremental Compilation {#skip}

While primarily focused on compile time, incremental compilation can also indirectly affect linking by reducing the amount of work the linker needs to do. Ensure it's enabled.

Incremental Linking: [Cargo][p~cargo]'s incremental compilation can help, but sometimes changes can invalidate the cache and require a full relink.

- [[incremental_computation | Incremental Computation]].

## Conditional Compilation {#skip}

This feature can improve compile times, especially for larger crates.

## Choosing the Right Linker {#skip}

The Rust compiler spends a lot of time in the "link" step. LLD is much faster at linking{{hi:Linking}} than the default Rust linker.

The default linker does a good job, but there are faster alternatives depending on the operating system you are using:

- [`lld`][lld~website]{{hi:lld}}↗ on Windows and Linux, a linker developed by the LLVM{{hi:LLVM}} project;
- [`zld`][zld~github]{{hi:zld}}↗ on MacOS. [![zld~github][zld~github~badge]][zld~github].

To speed up the linking phase you have to install the alternative linker on your machine and add this [configuration][p~configuration] file to the project:

```toml
# .cargo/config.toml
# On Windows
# ```
# Cargo install -f cargo-binutils
# Rustup Component add llvm-tools-preview
# ```
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# On Linux:
# - Ubuntu, `sudo apt-get Install lld clang`
# - Arch, `sudo Pacman -S lld clang`
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]

# On MacOS, `brew Install michaeleisel/zld/zld`
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
```

[`cargo-binutils`][c~cargo_binutils~docs]↗{{hi:cargo-binutils}} packages Cargo subcommands to invoke the LLVM tools shipped with the Rust toolchain.

### Alternative - `mold` Linker {#mold-linker}

[![cat~compilers][cat~compilers~badge]][cat~compilers]{{hi:Compilers}}

[`mold`][mold~github]{{hi:mold}}↗ is up to 5× faster than [`lld`][lld~website]{{hi:lld}}↗, but with a few caveats like limited platform support and occasional stability issues. To install [`mold`][c~mold~docs]↗{{hi:mold}}, run `sudo apt-get install mold clang` in Ubuntu.

You will also need to add the following to your [`cargo`][c~cargo~docs]{{hi:cargo}}↗ config at `.cargo/config.toml`:

```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=/usr/bin/mold"]
```

## Related Topics {#related-topics}

- [[development-tools | Development Tools]].
- [[development-tools_build-utils | Development Tools: Build Utils]].
- [[development-tools_cargo-plugins | Development Tools: Cargo Plugins]].
- [[development-tools_debugging | Development Tools" Debugging]].
- [[development-tools_ffi | Development Tools: FFI]].
- [[development-tools_procedural-macro-helpers | Development Tools Procedural Macro Helpers]].
- [[development-tools_testing | Development Tools Testing]].
- [[performance | Performance]].

## References {#references}

- [Enable Fast Compiles (Bevy)][c~bevy~enable-fast-compiles]{{hi:bevy}}↗.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[faster_linking: review - some linkers are deprecated](https://github.com/john-cd/rust_howto/issues/242)
</div>
