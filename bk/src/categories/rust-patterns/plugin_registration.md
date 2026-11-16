<!-- TODO Reviewed ??-??-2026. -->

# Register Commands Explicitly or with Inventory

For a small, closed command set, construct a list explicitly. This makes membership and initialization order visible. When independent crates must contribute commands without editing a central list, [`inventory`](https://docs.rs/inventory/latest/inventory/) can collect statically linked registrations. The example uses two crates: `plugin_registry` owns the declaration and `plugin_extension` submits `greet`.

```rust,ignore,noplayground
{{#include ../../../crates/cats/plugin_registry/examples/plugins.rs:example}}
```

Run `cargo test -p plugin_registry --example plugins` from `bk/crates`. The registry sorts by name before producing output because inventory iteration order is unspecified. It rejects a duplicate name instead of silently selecting whichever entry the linker visits first. Keep a linked reference to each extension crate; registration does not load a crate that is absent from the binary. Inventory uses startup constructors, and unusual linker settings, dynamic loading, and WebAssembly targets deserve a target-specific check before relying on discovery. Its [documentation](https://docs.rs/inventory/latest/inventory/) describes those caveats.

An enum is a straightforward choice for a closed set of commands, and a trait object can hold runtime-selected implementations. Generics are useful when the concrete type is known at compile time. Inventory changes how implementations are _found_ across linked crates; it does not replace the behavior interface. See [traits](../../language/traits.md), [trait objects and dynamic dispatch](../../language/trait_objects_and_dynamic_dispatch.md), and the [architecture draft](https://github.com/john-cd/rust_howto/tree/develop/later/src/other/architecture) for broader extension boundaries.
