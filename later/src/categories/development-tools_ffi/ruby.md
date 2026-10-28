<!-- TODO Reviewed ??-??-2026 -->

# Generate FFI Bindings to Ruby Code

{{#include ruby.incl.md}}

Interfacing with Ruby from Rust is typically done using the Ruby C API via a binding crate. For a new extension, [`magnus`][c~magnus~docs]↗{{hi:magnus}} supplies Ruby object conversions and method registration, backed by the lower-level [`rb-sys`][ffi~rb-sys] bindings and build tooling. Existing projects may also use Rutie; retain that option when its API and supported Ruby version match the project.

## Interop with Ruby via `rutie` {#rutie}

[![rutie][c~rutie~docs~badge]][c~rutie~docs] [![rutie~crates.io][c~rutie~crates.io~badge]][c~rutie~crates.io] [![rutie~repo][c~rutie~repo~badge]][c~rutie~repo] [![rutie~lib.rs][c~rutie~lib.rs~badge]][c~rutie~lib.rs]{{hi:rutie}}{{hi:Ruby}}{{hi:Cruby}}{{hi:rutie}}

The tie between Ruby and Rust.

### Windows support status {#ruby-windows-support}

The `magnus` and `rutie` examples are currently excluded on Windows by this
repository's platform gates. The manifest uses Magnus 0.9.3 and Rutie 0.14.3.
Rutie's [current support table][ffi~rutie-support] includes 64-bit Windows and
ties its release lines to particular Ruby versions; a local platform gate is
not a statement that upstream lacks Windows support. The devcontainer and
Windows dependency script install Ruby with development headers and a C
toolchain, but the bindings remain gated until their current feature builds and
runtimes are verified on Windows.

The [`rutie`][c~rutie~docs]↗{{hi:rutie}} crate wraps the Ruby C API. Check its exception and garbage-collection rules before retaining Ruby objects in Rust. Older guidance grouping Rutie with the legacy [Helix project][ffi~helix] as unmaintained is not a reliable description of Rutie's current documented releases.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/ruby/rutie.rs:example}}
```

## Write Ruby Extension gems in Rust, or call Ruby code from a Rust with `magnus` {#magnus}

[![magnus][c~magnus~docs~badge]][c~magnus~docs] [![magnus~crates.io][c~magnus~crates.io~badge]][c~magnus~crates.io] [![magnus~repo][c~magnus~repo~badge]][c~magnus~repo] [![magnus~lib.rs][c~magnus~lib.rs~badge]][c~magnus~lib.rs]{{hi:magnus}}{{hi:Ruby}}{{hi:Extension}}{{hi:Gem}}{{hi:Rubygem}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}} [![cat~development-tools::ffi][cat~development-tools::ffi~badge]][cat~development-tools::ffi]{{hi:FFI}}

High level Ruby bindings. Write Ruby extension gems in Rust, or call Ruby code from a Rust binary.

### Write a Native Extension {#ruby-native-extension}

In a Magnus extension library built as a `cdylib`, mark its registration function with `#[magnus::init]`. Use the supplied `&Ruby` handle to expose a plain Rust function with `define_global_function` and `function!`. Use `Result<T, magnus::Error>` for fallible methods so conversion failures become Ruby exceptions. This extension-only registration example is separate from the embedded executable below:

```rust,ignore
use magnus::{Error, Ruby, function};

fn hello_rust(name: String) -> String {
    format!("Hello, {name}!")
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    ruby.define_global_function("hello_rust", function!(hello_rust, 1))?;
    Ok(())
}
```

The [Magnus guide][ffi~magnus-guide] demonstrates extension setup and supported value conversions. Keep the greeting or other domain logic independent of the Ruby VM so ordinary Rust tests can exercise it.

### Embed Ruby in an Executable {#ruby-embedding}

Embedding is a different lifecycle: enable Magnus's [`embed` feature][ffi~magnus-embed] and initialize Ruby once on the thread that will use it. Keep the returned cleanup guard alive until all Ruby work finishes. Do not initialize a second VM or use Ruby values after cleanup. In an extension loaded by Ruby, the VM already exists, so do not initialize it again.

The included example below demonstrates embedding Ruby, registering a Rust function with `function!`, evaluating a Ruby call, and converting its result back to `String`. It is not an extension's registration function. Native embedding tests require Ruby headers and `libruby`, and run separately from the pure Rust greeting test.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/ruby/magnus.rs:example}}
```

## Other Less Common Options {#other-less-common-options}

- Directly using the Ruby C API is possible, but requires handling Ruby's exceptions, GC roots, and thread restrictions yourself. Prefer a binding layer whose contracts you can meet.
- Embedding Ruby: You can also embed a Ruby interpreter with Magnus or Rutie, respecting the selected binding's initialization contract.
- To call a small existing C ABI from Ruby, the [`ffi` gem][ffi~ruby-ffi] can load a Rust `cdylib` without Ruby-specific Rust bindings. Export C-compatible functions and define matching `attach_function` signatures. Specify allocation/free pairs and error results; Rust strings and `Vec` are not C ABI return types.

## Communication / Data Marshaling Between Ruby and Rust {#communication-data-marshaling-between-ruby-and-rust}

The [`rutie`][c~rutie~docs]↗{{hi:rutie}} crate handles much of the data marshaling between Rust and Ruby types. [`serde`][c~serde~docs]↗{{hi:serde}} can be used for serializing and deserializing data if needed.

Magnus also converts Rust arguments and return values directly. Keep Ruby objects reachable by the GC according to the binding's rules; a Rust heap container alone does not root an arbitrary Ruby `Value`. Do not access Ruby objects from an unrelated native thread, and respect Ruby's global VM lock. Serialize explicitly only when an application protocol or storage format needs it.

## Build Tools for Rust + Ruby {#build-tools-for-rust-ruby}

Use:

- [`cargo`][c~cargo~docs]↗{{hi:cargo}} for building the Rust library.
- [`rake`][c~rake~docs]↗{{hi:rake}} or `bundler` for managing the Ruby side and integrating with the Rust library.

With a Bundler version that supports Rust extensions and a Ruby development toolchain, scaffold and build a gem:

```bash
bundle gem --ext=rust hello_rust
cd hello_rust
bundle install
bundle exec rake compile
```

The scaffold connects Cargo with the `rb_sys` gem and Ruby's extension build process. Keep the gem's loader, native library name, and Magnus init symbol consistent. See the [RubyGems extension guide][ffi~ruby-extensions] and [`rb-sys`][ffi~rb-sys] for packaging. After defining `HelloRust.hello` in the extension, add a Ruby smoke test:

```ruby
require 'hello_rust'
raise 'unexpected greeting' unless HelloRust.hello('Ada') == 'Hello, Ada!'
```

From `later/crates`, `cargo test --locked -p development_tools_ffi --example ruby` checks the platform-independent greeting. On a configured non-Windows Ruby host, add `--features ruby` to compile native bindings. Run the ignored embedding tests individually in separate processes, since each initializes a VM:

```bash
cargo test --locked -p development_tools_ffi --example ruby --features ruby embedded_ruby_returns_the_rust_greeting -- --ignored --test-threads=1
cargo test --locked -p development_tools_ffi --example ruby --features ruby test_rutie_example_entry_point -- --ignored --test-threads=1
```

An ignored test still needs Ruby headers/libraries to compile and link. A Windows pass exercises only the pure helper, not either native binding; test gem loading with the intended Ruby version before release.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
