<!-- TODO Reviewed ??-??-2026. -->

# Dependencies

{{#include dependencies.incl.md}}

## Use Items from External Crates (Dependencies) in your Code {#dependencies}

To use a (public) item (structs, enums, traits, functions, etc.) of an external crate, first make sure to include the external crate as a dependency{{hi:Dependency}} in your crate's [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}} file, for example:

```toml
[dependencies]
anyhow = "1.0.104"
```

The name of the external crate is followed by `=` and a semantic version specifier. See the [Cargo](../categories/development-tools/cargo/cargo.md) chapters for details.

You can then refer to its items using a path that starts with the external crate's name, e.g. `anyhow::Result`.

The path may include one or more modules as needed e.g., `<crate_name>::<module_name>::<item_name>`.

You may start the path with `::` followed by the name of the crate if a crate name is the same as a module in your crate.

Once adding an external crate as a dependency, you will very often bring items from external crates into scope with the `use` keyword. See the [[use_declarations | `use` Keyword]] chapter for details. The following example uses both styles:

```rust
{{#include ../../crates/code_organization/examples/dependencies/external_dependency.rs:example}}
```

For lockfiles, feature selection, and dependency updates, see [Dependency Management](../categories/development-tools_cargo-plugins/dependency_management.md).

## Check for Unused Dependencies with Cargo 1.100

Cargo 1.100 reports `cargo::unused_dependencies` when it can determine that a dependency table entry is unused. From the workspace root, run `cargo +1.100.0 check --workspace --all-targets --locked` before removing a candidate. A targeted `--bin` build cannot rule out another binary using a normal dependency; Cargo does not currently lint unused `dev-dependencies`, even with `--all-targets`. A dependency used only to activate a transitive feature can also be a false positive. Check feature and target combinations before editing the manifest, and rerun tests after a removal. The [Cargo lint reference](https://doc.rust-lang.org/cargo/reference/lints.html#unused_dependencies) describes the exact coverage and exceptions.

## Delay New Registry Releases When Resolving

Cargo 1.100 supports a minimum publish age for registry versions. A project that wants a three-day review window can put this in `.cargo/config.toml`:

```toml
[registry]
global-min-publish-age = "3 days"
```

The setting affects **new resolution**: an already locked version remains in `Cargo.lock`, and a deliberate `cargo update --precise` can be used when a reviewed fix is needed sooner. Commit and review lockfile changes. The policy is not an advisory scanner or a substitute for checking dependency provenance; it also depends on the registry recording publication times. `cargo install` does not apply this cooldown. See the [Rust 1.100 announcement](https://blog.rust-lang.org/releases/1.100.0/) and [Cargo configuration reference](https://doc.rust-lang.org/cargo/reference/config.html#registryglobal-min-publish-age).

Cargo 1.100 also reorganizes intermediate build artifacts under the configured build directory by build-configuration hash. Different features or compiler inputs can coexist with fewer collisions, but a target directory remains local build state rather than a portable dependency cache. Keep build outputs separate between checkout-specific worktrees and measure any cache policy before changing it. See [Cargo's build cache reference](https://doc.rust-lang.org/cargo/reference/build-cache.html) and the [build-time diagnosis](../categories/development-tools/build_time_diagnosis.md) chapter.

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
