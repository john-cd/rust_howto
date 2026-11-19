<!-- TODO Reviewed ??-??-2026. -->

# Development Tools Procedural Macro Helpers

This crate contains code examples for the **Development Tools Procedural Macro Helpers** category on crates.io. It demonstrates integration with key ecosystem crates including `anyhow`, `clap`, `darling`, `paste`, `prettyplease`, `proc-macro2`.

## Examples Covered

- **Compile Macros** (`examples/compile_macros/`): Exercises token transformations on the host. See the [Watt walkthrough](../../../src/categories/development-tools_procedural-macro-helpers/compile_macros.md) to build the guest and run the separate `watt_round_trip/` integration test.
- **Write Proc Macros** (`examples/write_proc_macros/`): Demonstrates concepts like `darling`, `paste`, `proc_macro2`, `quote`, `syn`.

## Usage

To check or run examples in this crate:

```console
cargo check --package development_tools_procedural_macro_helpers --examples
cargo test --locked -p development_tools_procedural_macro_helpers --examples
cargo test --locked -p development_tools_procedural_macro_helpers --doc
```

Code examples correspond to chapters in `bk/src/categories/development-tools_procedural-macro-helpers` or related book sections.
