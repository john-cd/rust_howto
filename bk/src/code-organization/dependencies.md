<!-- TODO Reviewed ??-??-2026. -->

# Dependencies

{{#include dependencies.incl.md}}

## Use Items from External Crates (Dependencies) in your Code {#dependencies}

To use a (public) item (structs, enums, traits, functions, etc.) of an external crate, first make sure to include the external crate as a dependency{{hi:Dependency}} in your crate's [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}} file, for example:

```toml
[dependencies]
anyhow = "1.0.104"
```

The name of the external crate is followed by `=` and a semantic version specifier. See the [[cargo | Cargo]] chapters for details.

You can then refer to its items using a path that starts with the external crate's name, e.g. `anyhow::Result`.

The path may include one or more modules as needed e.g., `<crate_name>::<module_name>::<item_name>`.

You may start the path with `::` followed by the name of the crate if a crate name is the same as a module in your crate.

Once adding an external crate as a dependency, you will very often bring items from external crates into scope with the `use` keyword. See the [[use_declarations | `use` Keyword]] chapter for details. The following example uses both styles:

```rust
{{#include ../../crates/code_organization/examples/dependencies/external_dependency.rs:example}}
```

For lockfiles, feature selection, and dependency updates, see [Dependency Management (draft)](https://github.com/john-cd/rust_howto/blob/develop/bk/drafts/categories/development-tools_cargo-plugins/dependency_management.md).

## References {#references .skip}

- [Using a Crate to Get More Functionality][book~rust~ch02-00-guessing-game-tutorial.html#using-a-crate-to-get-more-functionality]↗.

## Related Topics {#related-topics .skip}

- [[code_organization_by_project_type | Code Organization By Project Type]].
- [[crate-selection | Crate Selection]].

See [No Standard Library](../categories/no-std/no_std.md) for a library that uses a feature to select `std` support.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
  - [[development-tools_cargo-plugins | Development Tools - Cargo Plugins]].
</div>
