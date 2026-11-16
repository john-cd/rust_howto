<!-- TODO Reviewed ??-??-2026. -->

# Diagnose Rust Build Time

Measure the build that slows your edit loop before changing a profile, linker, or dependency. From this repository's root, a focused example is:

```console
cargo build --locked --manifest-path bk/crates/Cargo.toml -p standard_library --timings
cargo tree --manifest-path bk/crates/Cargo.toml -p standard_library --duplicates
cargo tree --manifest-path bk/crates/Cargo.toml -p standard_library -e features
```

Cargo prints the timing report path under the configured target directory's `cargo-timings` folder. The report separates crate compilation from the critical path; the terminal output also shows whether linking and other build steps dominate. Look for duplicated versions or unnecessary features before changing the dependency graph. The [Cargo timings guide](https://doc.rust-lang.org/cargo/reference/timings.html) explains the report, and the [Corrode walkthrough](https://corrode.dev/blog/tips-for-faster-rust-compile-times/#find-the-slow-crate-in-your-codebase) shows how to find a slow crate.

On the Windows validation host `J-RAZER` (20 logical processors, `x86_64-pc-windows-msvc`, Rust/Cargo 1.100.0), `cargo build --locked -p standard_library --timings` with this book's manifest and warm shared dependencies took **1.86 s** on the first measured run and **0.60 s** on an immediate no-op repeat on November 16, 2026. These wall times include Cargo startup and are observations, not speed targets. The report was written under `D:\target\rust_howto\d\cargo-timings`. A cold build with an empty checkout-specific target directory and a one-file edited build answer different questions; record all three separately when investigating your project. Do not clear a shared compiler cache to simulate a cold build.

Change one variable at a time, then repeat the same workload. Development profiles trade debug information and runtime speed for compilation speed. Incremental artifacts help many edited builds, while `sccache` can reuse matching compiler work across clean builds; inspect cache hit statistics before crediting either. A linker change is worth keeping only after measuring its effect on the project's target and profile. [Feldera's case study](https://www.feldera.com/blog/cutting-down-rust-compile-times-from-30-to-2-minutes-with-one-thousand-crates) shows how dependency choices can dominate. The [draft compilation guide](https://github.com/john-cd/rust_howto/blob/develop/bk/drafts/categories/development-tools/compilation/reduce_compilation_duration.md) covers profile, nightly, and linker options in more detail.

For repeatable team measurements, a small [`cargo xtask`](https://github.com/matklad/cargo-xtask) command can pin the package, features, profile, and output path. If old outputs need removal, [`cargo-clean-all`](https://github.com/dnlmlr/cargo-clean-all) can list candidates; review its selected root and interactive choices before deleting anything. Keep each checkout's build output separate and retain shared package caches.
