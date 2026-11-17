<!-- TODO Reviewed ??-??-2026. -->

# Configure Code Based on Compiler Support

{{#include autocfg.incl.md}}

## Detect Compiler Features with `autocfg` {#autocfg}

{{#badges autocfg development-tools::build-utils="Build Utils"}}{{hi:Autoconf}}{{hi:Build}}{{hi:Rustc}}

[`autocfg`][c~autocfg~docs] is a library for Cargo build scripts. It compiles small probes with the compiler Cargo is using, then reports which probes succeed as custom [`cfg` settings][book~rust-reference~conditional-compilation]. This lets a crate choose a compatible implementation based on the APIs the compiler actually accepts.

Use a probe when compatibility depends on a particular type, trait, path, expression, or constant. A Rust version check can miss backports or toolchain differences, while a successful probe confirms that the tested code compiles. `autocfg` provides helpers such as `emit_has_type`, `emit_has_trait`, and `emit_expression_cfg`; use `probe_raw` when the feature needs a larger or crate-level snippet.

Cargo features serve a different purpose: they let a package's users opt into functionality. Compiler probes select code that the active compiler can build. For target facts such as the operating system or architecture, read Cargo's `CARGO_CFG_*` environment variables in the build script; `cfg!` there describes the host running the script, which can differ from the target during cross-compilation.

### Add a build dependency {#autocfg-build-dependency .skip}

Declare `autocfg` under `[build-dependencies]`, because it runs in `build.rs`:

```toml
[build-dependencies]
autocfg = "1"
```

### Probe and emit a custom `cfg` {#autocfg-build-script .skip}

The example is a standalone probe that reports whether `i128` is available. In a real `build.rs`, use `emit_has_type` to emit a custom `cfg` and declare it as expected so Cargo's `unexpected_cfgs` lint can check its use. Cargo interprets `cargo::` instructions only when they are printed by a build script.

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/examples/autocfg/autocfg.rs:example}}
```

In a real `build.rs`, Cargo sets `OUT_DIR`, which `autocfg::new()` uses for probe output. The checked standalone example uses `AutoCfg::with_dir` so it can fall back to the system temporary directory when run outside a build script.

```rust
fn main() {
    println!("cargo::rustc-check-cfg=cfg(has_i128)");
    autocfg::new().emit_has_type("i128");
}
```

Use the emitted setting in the crate's source to select an implementation:

```rust
#[cfg(has_i128)]
type WideInteger = i128;

#[cfg(not(has_i128))]
type WideInteger = i64;
```

Keep the fallback branch valid for every compiler that can reach it. The probe only checks that code compiles; it does not prove that an API has the runtime behavior your crate needs.

## Related Topics {#related-topics .skip}

| Topic | Description |
| --- | --- |
| [Cargo build scripts][book~cargo~build-script] | Run build-time checks and pass instructions to Cargo before compiling a package. |
| [Conditional compilation][book~rust-reference~conditional-compilation] | Use the emitted `cfg` setting to select code at compile time. |
| [Cargo](https://doc.rust-lang.org/cargo/reference/features.html) | Configure user-selectable package features separately from compiler capability probes. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
