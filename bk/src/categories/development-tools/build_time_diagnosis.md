<!-- TODO Reviewed ??-??-2026. -->

# Diagnose Rust Build Time

Measure the build that slows your edit loop before changing a profile, linker, or dependency. From the repository root, run the following commands to measure and inspect a single package.

```console
cargo build --locked --manifest-path bk/crates/Cargo.toml -p standard_library --timings
cargo tree --manifest-path bk/crates/Cargo.toml -p standard_library --duplicates
cargo tree --manifest-path bk/crates/Cargo.toml -p standard_library -e features
```

Cargo prints the timing report path under the configured target directory's `cargo-timings` folder. The report separates crate compilation from the critical path; the terminal output also shows whether linking and other build steps dominate. Look for duplicated versions or unnecessary features before changing the dependency graph. The [Cargo timings guide](https://doc.rust-lang.org/cargo/reference/timings.html) explains the report, and the [Corrode walkthrough](https://corrode.dev/blog/tips-for-faster-rust-compile-times/#find-the-slow-crate-in-your-codebase) shows how to find a slow crate.

Measure three scenarios separately. Start with an empty checkout-specific target directory, repeat the build without edits, then edit one source file and rebuild. Record whether the shared compiler cache was warm and keep the same cache settings throughout. Use wall time to measure what you wait for, including Cargo startup.

Change one variable at a time, then repeat the same workload. Development profiles trade debug information and runtime speed for compilation speed. Incremental artifacts help many edited builds, while `sccache` can reuse matching compiler work across clean builds; inspect cache hit statistics before crediting either. A linker change is worth keeping only after measuring its effect on the project's target and profile. [Feldera's case study](https://www.feldera.com/blog/cutting-down-rust-compile-times-from-30-to-2-minutes-with-one-thousand-crates) shows how dependency choices can dominate. [Reduce Compilation Duration](compilation/reduce_compilation_duration.md) covers profile, nightly, and linker options in more detail.

For repeatable team measurements, a small [`cargo xtask`](https://github.com/matklad/cargo-xtask) command can pin the package, features, profile, and output path. If old outputs need removal, [`cargo-clean-all`](https://github.com/dnlmlr/cargo-clean-all) can list candidates; review its selected root and interactive choices before deleting anything. Keep each checkout's build output separate and retain shared package caches.
