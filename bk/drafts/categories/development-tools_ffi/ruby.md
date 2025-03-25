# Generate FFI Bindings to Ruby Code

{{#include ruby.incl.md}}

Interfacing with Ruby from Rust is typically done using the Ruby C API via a binding crate. [`rutie`][c-rutie]⮳{{hi:rutie}} is the key crate here. It significantly simplifies the process of creating Ruby extensions in Rust. It's highly recommended to use it unless you have very specific low-level Ruby C API requirements. It abstracts away much of the complexity of manual Ruby C API interaction.

## Interop with Ruby via `rutie` {#rutie}

[![rutie][c-rutie-badge]][c-rutie] [![rutie-crates.io][c-rutie-crates.io-badge]][c-rutie-crates.io] [![rutie-github][c-rutie-github-badge]][c-rutie-github] [![rutie-lib.rs][c-rutie-lib.rs-badge]][c-rutie-lib.rs]{{hi:rutie}}{{hi:Ruby}}{{hi:Cruby}}{{hi:rutie}}

The tie between Ruby and Rust.

The [`rutie`][c-rutie]⮳{{hi:rutie}} crate provides a high-level and safe interface to the Ruby C API. It's the most common and recommended approach for Ruby/Rust integration. It handles a lot of the boilerplate and memory management.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/tests/ruby/rutie.rs:example}}
```

## Write Ruby Extension gems in Rust, or call Ruby code from a Rust with `magnus` {#magnus}

[![magnus][c-magnus-badge]][c-magnus] [![magnus-crates.io][c-magnus-crates.io-badge]][c-magnus-crates.io] [![magnus-github][c-magnus-github-badge]][c-magnus-github] [![magnus-lib.rs][c-magnus-lib.rs-badge]][c-magnus-lib.rs]{{hi:magnus}}{{hi:Ruby}}{{hi:Extension}}{{hi:Gem}}{{hi:Rubygem}} [![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}} [![cat-development-tools::ffi][cat-development-tools::ffi-badge]][cat-development-tools::ffi]{{hi:FFI}}

High level Ruby bindings. Write Ruby extension gems in Rust, or call Ruby code from a Rust binary.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/tests/ruby/magnus.rs:example}}
```

## Other Less Common Options {#skip}

- Directly using the Ruby C API is possible, but it's complex and error-prone. [`rutie`][c-rutie]⮳{{hi:rutie}} is almost always the better choice.
- Embedding Ruby: You can also embed a Ruby interpreter in your Rust application using the Ruby C API (and thus, [`rutie`][c-rutie]⮳{{hi:rutie}}).

## Communication / Data Marshaling Between Ruby and Rust {#skip1}

The [`rutie`][c-rutie]⮳{{hi:rutie}} crate handles much of the data marshaling between Rust and Ruby types. [`serde`][c-serde]⮳{{hi:serde}} can be used for serializing and deserializing data if needed, but often [`rutie`][c-rutie]⮳{{hi:rutie}}'s built-in conversion mechanisms are sufficient.

## Build Tools for Rust + Ruby {#skip2}

Use:

- [`cargo`][c-cargo]⮳{{hi:cargo}} for building the Rust library.
- `rake` or `bundler` for managing the Ruby side and integrating with the Rust library.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1076)
</div>
