<!-- TODO Reviewed ??-??-2026 -->
# Generate FFI Bindings to Ruby Code

{{#include ruby.incl.md}}

Interfacing with Ruby from Rust is typically done using the Ruby C API via a binding crate. [`rutie`][c~rutie~docs]↗{{hi:rutie}} is the key crate here. It significantly simplifies the process of creating Ruby extensions in Rust.

## Interop with Ruby via `rutie` {#rutie}

[![rutie][c~rutie~docs~badge]][c~rutie~docs] [![rutie~crates.io][c~rutie~crates.io~badge]][c~rutie~crates.io] [![rutie~repo][c~rutie~repo~badge]][c~rutie~repo] [![rutie~lib.rs][c~rutie~lib.rs~badge]][c~rutie~lib.rs]{{hi:rutie}}{{hi:Ruby}}{{hi:Cruby}}{{hi:rutie}}

The tie between Ruby and Rust.

### Windows support status {#ruby-windows-support}

The `ruby` Cargo feature and its `magnus` and `rutie` examples are currently excluded on Windows by `cfg(not(windows))` in this repository. The manifest pins Rutie 0.9.0, while the upstream support table identifies Windows for the Ruby 2 line starting at Rutie 0.10.2. The upstream Windows setup also requires 64-bit Ruby built with MinGW and the Visual Studio C++ Build Tools. This checkout has no Ruby installation, so Windows compilation and runtime support are not verified here. Keep the examples on their current platform gate until compatible versions and that toolchain are exercised. See the [Rutie Windows requirements](https://github.com/danielpclark/rutie#operating-system-requirements).

The [`rutie`][c~rutie~docs]↗{{hi:rutie}} crate provides a high-level and safe interface to the Ruby C API. It's the most common and recommended approach for Ruby/Rust integration.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/ruby/rutie.rs:example}}
```

## Write Ruby Extension gems in Rust, or call Ruby code from a Rust with `magnus` {#magnus}

[![magnus][c~magnus~docs~badge]][c~magnus~docs] [![magnus~crates.io][c~magnus~crates.io~badge]][c~magnus~crates.io] [![magnus~repo][c~magnus~repo~badge]][c~magnus~repo] [![magnus~lib.rs][c~magnus~lib.rs~badge]][c~magnus~lib.rs]{{hi:magnus}}{{hi:Ruby}}{{hi:Extension}}{{hi:Gem}}{{hi:Rubygem}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}} [![cat~development-tools::ffi][cat~development-tools::ffi~badge]][cat~development-tools::ffi]{{hi:FFI}}

High level Ruby bindings. Write Ruby extension gems in Rust, or call Ruby code from a Rust binary.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/ruby/magnus.rs:example}}
```

## Other Less Common Options {#other-less-common-options}

- Directly using the Ruby C API is possible, but it's complex and error-prone. [`rutie`][c~rutie~docs]↗{{hi:rutie}} is almost always the better choice.
- Embedding Ruby: You can also embed a Ruby interpreter in your Rust application using the Ruby C API (and thus, [`rutie`][c~rutie~docs]↗{{hi:rutie}}).

## Communication / Data Marshaling Between Ruby and Rust {#communication-data-marshaling-between-ruby-and-rust}

The [`rutie`][c~rutie~docs]↗{{hi:rutie}} crate handles much of the data marshaling between Rust and Ruby types. [`serde`][c~serde~docs]↗{{hi:serde}} can be used for serializing and deserializing data if needed.

## Build Tools for Rust + Ruby {#build-tools-for-rust-ruby}

Use:

- [`cargo`][c~cargo~docs]↗{{hi:cargo}} for building the Rust library.
- [`rake`][c~rake~docs]↗{{hi:rake}} or `bundler` for managing the Ruby side and integrating with the Rust library.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1076)
</div>
