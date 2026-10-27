<!-- TODO Reviewed ??-??-2026 -->

# Generate FFI Bindings to Erlang / Elixir Code

{{#include erlang_elixir.incl.md}}

Interfacing with Erlang/Elixir from Rust typically involves using ports or NIFs (Native Implemented Functions).

## Ports {#ports}

'Ports' are the simpler approach. Rust communicates with Erlang/Elixir as a separate process using standard input/output. No specific crate is required, but you'll use [`std::process`][c~std::process~docs]↗{{hi:std::process}} in Rust and the port mechanisms in Erlang/Elixir.

Use a port when a native crash must be isolated from the BEAM. Define message framing, bound input sizes, and handle process exits. C nodes are another separate-process option when Erlang's distribution protocol is appropriate; configure distribution authentication and network access deliberately.

## NIFs (Native Implemented Functions) {#nifs}

Native Implemented Functions (NIFs) allow you to write functions in Rust that are called by the Erlang VM as if they were native Erlang functions.

## `rustler` {#rustler}

[![rustler][c~rustler~docs~badge]][c~rustler~docs] [![rustler~crates.io][c~rustler~crates.io~badge]][c~rustler~crates.io] [![rustler~repo][c~rustler~repo~badge]][c~rustler~repo] [![rustler~lib.rs][c~rustler~lib.rs~badge]][c~rustler~lib.rs]{{hi:rustler}}

[`rustler`][c~rustler~docs]↗{{hi:rustler}} provides safe Rust wrappers for creating Erlang NIF functions.

Create a native crate with `mix rustler.new` after adding the Rustler Mix dependency, and build it as a `cdylib`. Mark exported functions with `#[rustler::nif]` and register the Elixir module with `rustler::init!("Elixir.MyRustLibrary")`. Match that name in the loader and provide a fallback for each exported function:

```elixir
defmodule MyRustLibrary do
  use Rustler, otp_app: :my_app, crate: :my_rust_library

  def greet(_name), do: :erlang.nif_error(:nif_not_loaded)
  def add(_left, _right), do: :erlang.nif_error(:nif_not_loaded)
end
```

Replace the application and crate names with those generated for your project. The [Rustler Mix guide][ffi~rustler-mix] explains dependency setup and loading. The included example uses Rustler 0.39 and separates ordinary Rust logic from NIF wrappers:

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/erlang_elixir/rustler.rs:example}}
```

## Communication/Serialization (for both Ports and NIFs) {#communication-serialization}

Use the following crates:

- [[serde | `serde`]] for serializing and deserializing data exchanged between Rust and Erlang/Elixir.
- [`bincode`][c~bincode~docs]↗{{hi:bincode}} is a compact binary format that is often used for efficient communication.
- [`erlang_term`][c~erlang-term~docs]{{hi:erlang_term}} converts Erlang External Term Format to Rust objects, without using Erlang NIFs.

Within a NIF, Rustler's `Encoder` and `Decoder` convert terms directly; a separate serialization format is unnecessary for ordinary integers, strings, tuples, or maps. Use existing atoms for a fixed protocol vocabulary, `#[derive(NifStruct)]` with its matching Elixir module for structs, or `#[derive(NifMap)]` for maps. Do not create an unbounded number of atoms from user input. `serde` and a binary format are options only when both ends explicitly agree on the wire format; BEAM does not decode `bincode` automatically.

### Schedule Long Work and Retain State {#nif-scheduling-and-state}

Keep ordinary NIFs short. Use [`#[rustler::nif(schedule = "DirtyCpu")]`][ffi~rustler-nif] for sustained CPU work and `DirtyIo` for blocking IO. Dirty schedulers protect normal scheduler responsiveness, but do not isolate crashes or provide unlimited worker capacity. Bound input, memory use, and concurrent jobs.

Use [`ResourceArc<T>`][ffi~rustler-resource] for native state shared with BEAM terms. Implement and register Rustler's `Resource` type, then give Elixir a resource reference rather than an integer containing a raw pointer. Shared mutation requires synchronization, and destructors must release native resources safely. Ordinary `Term` and `Env` values belong to a call's environment; do not store them as long-lived Rust references.

## Build Tools {#build-tools .skip}

- [[cargo | `cargo`]] builds the Rust side of the integration.
- [`mix`][mix~website]↗{{hi:mix}} builds the Elixir side and manages dependencies.

For release distribution, [`rustler_precompiled`][ffi~rustler-precompiled] downloads platform-specific NIF artifacts and checks their checksums. Publish artifacts for the supported target triples and NIF ABI versions, retain a documented source-build fallback, and exercise loading in `mix test` on each supported runtime.

From `later/crates`, run `cargo test --locked -p development_tools_ffi --example erlang_elixir` for the pure greeting and arithmetic tests. Add `--features erlang` to compile the NIF wrappers. These Rust tests do not load a NIF into BEAM; loading and Elixir term-conversion tests belong in the generated Mix project.

## Other Considerations {#other-considerations .skip}

- Careful [error handling][p~error-handling] is crucial, especially with NIFs, as a crash in the NIF can bring down the entire Erlang VM.
- Erlang/Elixir and Rust have different [concurrency][p~concurrency] models. You'll need to be mindful of how you manage [concurrency][p~concurrency] across the FFI boundary.
- You'll need to map data types between Rust and Erlang/Elixir. The [`erlang_term`][c~erlang-term~docs]{{hi:erlang_term}} crate can help with this.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
