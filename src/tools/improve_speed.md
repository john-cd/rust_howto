# Reduce compilation duration

Rust compile times can be long.

## Measuring build times

```sh
time cargo build
```

```sh
cargo build --timings
```

## Dynamic linking

```sh
cargo install cargo-add-dynamic
cargo add-dynamic polars --features csv-file,lazy,list,describe,rows,fmt,strings,temporal
cargo build
```

[Speeding up incremental Rust compilation with dylibs]( https://robert.kra.hn/posts/2022-09-09-speeding-up-incremental-rust-compilation-with-dylibs/ )

## Incremental Compilation

From-scratch builds with incremental compilation enabled adds about 15–20% overhead compared to disabled. The initial build needs to write out more intermediate state in order for later incremental builds to take advantage of it.

## Reference

[8 Solutions for Troubleshooting Your Rust Build Times]( https://jondot.medium.com/8-steps-for-troubleshooting-your-rust-build-times-2ffc965fd13e )
