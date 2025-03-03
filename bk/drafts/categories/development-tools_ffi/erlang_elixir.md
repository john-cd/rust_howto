# Generate FFI bindings to Erlang / Elixir code

{{#include erlang_elixir.incl.md}}

## `rustler` {#rustler}

[![rustler][c-rustler-badge]][c-rustler] [![rustler-crates.io][c-rustler-crates.io-badge]][c-rustler-crates.io] [![rustler-github][c-rustler-github-badge]][c-rustler-github] [![rustler-lib.rs][c-rustler-lib.rs-badge]][c-rustler-lib.rs]{{hi:rustler}}

[`rustler`][c-rustler]⮳{{hi:rustler}} provides safe Rust wrappers for creating Erlang NIF functions.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/tests/erlang_elixir/rustler.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1070)

Interfacing with Erlang/Elixir from Rust typically involves using ports or NIFs (Native Implemented Functions). Here's a breakdown:

Ports:

(Ports are the simpler approach. Rust communicates with Erlang/Elixir as a separate process using standard input/output. No specific crate is required, but you'll use std::process in Rust and the port mechanisms in Erlang/Elixir.)
NIFs (Native Implemented Functions):

erlang_nif: This crate allows you to write NIFs in Rust. NIFs run within the Erlang VM, so they are more performant than ports, but also more complex and require extra care to avoid crashing the VM.
Communication/Serialization (for both ports and NIFs):

serde: (Not Erlang/Elixir specific, but essential for serializing and deserializing data exchanged between Rust and Erlang/Elixir.)
bincode: A compact binary format that is often used for efficient communication.
erlang_term: A crate for working with Erlang terms.
Build Tools:

cargo: (Essential for building the Rust side of the integration.)
rebar3 or mix: (For building the Erlang/Elixir side and managing dependencies.)
Other Considerations:

Error Handling: Careful [error handling][p-error-handling] is crucial, especially with NIFs, as a crash in the NIF can bring down the entire Erlang VM.
Concurrency: Erlang/Elixir and Rust have different concurrency models. You'll need to be mindful of how you manage [concurrency][p-concurrency] across the FFI boundary.
Data Types: You'll need to map data types between Rust and Erlang/Elixir. The erlang_term crate can help with this.
The erlang_nif crate is the key for writing NIFs. For ports, you'll primarily use standard Rust I/O mechanisms. Serialization libraries are crucial for efficient data exchange in both cases.
</div>
