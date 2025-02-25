# Generate FFI bindings to Ruby code

{{#include ruby.incl.md}}

## Magnus {#magnus}

[![magnus][c-magnus-badge]][c-magnus] [![magnus-crates.io][c-magnus-crates.io-badge]][c-magnus-crates.io] [![magnus-github][c-magnus-github-badge]][c-magnus-github] [![magnus-lib.rs][c-magnus-lib.rs-badge]][c-magnus-lib.rs]{{hi:magnus}}{{hi:Ruby}}{{hi:Extension}}{{hi:Gem}}{{hi:Rubygem}} [![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}} [![cat-development-tools::ffi][cat-development-tools::ffi-badge]][cat-development-tools::ffi]{{hi:FFI}}

High level Ruby bindings. Write Ruby extension gems in Rust, or call Ruby code from a Rust binary.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/tests/ruby/magnus.rs:example}}
```

## Rutie {#rutie}

[![rutie][c-rutie-badge]][c-rutie] [![rutie-crates.io][c-rutie-crates.io-badge]][c-rutie-crates.io] [![rutie-github][c-rutie-github-badge]][c-rutie-github] [![rutie-lib.rs][c-rutie-lib.rs-badge]][c-rutie-lib.rs]{{hi:rutie}}{{hi:Ruby}}{{hi:Cruby}}{{hi:rutie}}

The tie between Ruby and Rust.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/tests/ruby/rutie.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 write](https://github.com/john-cd/rust_howto/issues/1076)

Interfacing with Ruby from Rust is typically done using the Ruby C API. Here's a breakdown:

## Ruby C API Bindings

rutie: This crate provides a high-level and safe interface to the Ruby C API. It's the most common and recommended approach for Ruby/Rust integration. It handles a lot of the boilerplate and memory management.
Other (Less Common) Options:

Directly using the Ruby C API is possible, but it's complex and error-prone. rutie is almost always the better choice.

## Communication/Data Marshaling

The rutie crate handles much of the data marshaling between Rust and Ruby types.)

serde: (Can be used for serializing and deserializing data if needed, but often rutie's built-in conversion mechanisms are sufficient.)

## Build Tools

cargo: (For building the Rust library.)
rake or bundler: (For managing the Ruby side and integrating with the Rust library.)

## Key Concepts and Workflow

Ruby C API: You'll use the rutie crate to interact with the Ruby C API.
Gems: You'll typically create a Ruby gem that includes your Rust library.
Data Marshaling: You'll need to convert data between Rust and Ruby types. rutie greatly simplifies this.
Garbage Collection: Ruby has its own garbage collector. rutie helps to manage this interaction and prevent memory leaks.
Embedding Ruby: You can also embed a Ruby interpreter in your Rust application using the Ruby C API (and thus, rutie).

rutie is the key crate here. It significantly simplifies the process of creating Ruby extensions in Rust. It's highly recommended to use it unless you have very specific low-level Ruby C API requirements. It abstracts away much of the complexity of manual Ruby C API interaction.
</div>
