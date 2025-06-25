# Generate FFI Bindings to Erlang / Elixir Code

{{#include erlang_elixir.incl.md}}

Interfacing with Erlang/Elixir from Rust typically involves using ports or NIFs (Native Implemented Functions).

## Ports {#skip}

'Ports' are the simpler approach. Rust communicates with Erlang/Elixir as a separate process using standard input/output. No specific crate is required, but you'll use `std::process` in Rust and the port mechanisms in Erlang/Elixir.

## NIFs (Native Implemented Functions) {#skip}

The `erlang_nif` crate allows you to write NIFs in Rust. NIFs run within the Erlang VM, so they are more performant than ports, but also more complex and require extra care to avoid crashing the VM.

## `rustler` {#rustler}

[![rustler][c-rustler-badge]][c-rustler] [![rustler-crates.io][c-rustler-crates.io-badge]][c-rustler-crates.io] [![rustler-github][c-rustler-github-badge]][c-rustler-github] [![rustler-lib.rs][c-rustler-lib.rs-badge]][c-rustler-lib.rs]{{hi:rustler}}

[`rustler`][c-rustler]⮳{{hi:rustler}} provides safe Rust wrappers for creating Erlang NIF functions.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/erlang_elixir/rustler.rs:example}}
```

## Communication/Serialization (for both Ports and NIFs) {#skip}

Use the following crates:

- [[serde | `serde`]] for serializing and deserializing data exchanged between Rust and Erlang/Elixir.
- [`bincode`][c-bincode]⮳{{hi:bincode}} is a compact binary format that is often used for efficient communication.
- `erlang_term` converts Erlang External Term Format to Rust objects, without using Erlang NIFs.

## Build Tools {#skip}

- [[cargo | `cargo`]] builds the Rust side of the integration.
- `rebar3` or `mix` build the Erlang/Elixir side and manage dependencies.

## Other Considerations {#skip}

- Careful [error handling][p-error-handling] is crucial, especially with NIFs, as a crash in the NIF can bring down the entire Erlang VM.
- Erlang/Elixir and Rust have different [concurrency][p-concurrency] models. You'll need to be mindful of how you manage [concurrency][p-concurrency] across the FFI boundary.- You'll need to map data types between Rust and Erlang/Elixir. The `erlang_term` crate can help with this.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1070)
review in depth
</div>
