<!-- TODO Reviewed ??-??-2026. -->

# Configure Clippy for a Workspace

`cargo clippy` catches patterns the compiler accepts, such as holding a lock across `.await` or dropping a future by assigning it to `_`. Put lint levels in Cargo manifests so local runs and CI start from the same policy. Pick additional lints for mistakes that matter to your project. [Evan Schwartz's stricter Clippy configuration][blog~stricter-clippy]↗ groups opt-in lints by the failure they help catch.

{{#include clippy.incl.md}}

## Share Lint Levels Across Packages {#workspace-lints}

Add a lint policy to the workspace root's `Cargo.toml`.

```toml
[workspace]
members = ["app"]
resolver = "3"

[workspace.lints.rust]
unsafe_op_in_unsafe_fn = "warn"

[workspace.lints.clippy]
unwrap_used = "warn"
expect_used = "warn"
await_holding_lock = "warn"
let_underscore_future = "warn"
```

Then opt in from each member's `Cargo.toml`. A member that inherits the workspace table must put any local exceptions in its source code.

```toml
[package]
name = "app"
version = "0.1.0"
edition = "2024"

[lints]
workspace = true
```

Cargo accepts an integer `priority` in a lint entry. If you opt into a group and want an individual lint to win, give the group a lower priority. For example, `pedantic = { level = "warn", priority = -1 }` lets an individual setting at the default priority of zero take precedence. Keep Clippy's default correctness levels in a new project and review any group you enable. Clippy's restriction lints are intended for individual selection. See [Cargo workspace inheritance][cargo~workspace-lints]↗ and [Clippy's lint groups][book~clippy~lint-groups]↗.

## Configure a Lint and Justify Exceptions {#lint-exceptions}

`clippy.toml` holds lint parameters. In the workspace directory, this setting lets a test use `expect` to explain a fixture failure. Production code still gets the `expect_used` diagnostic.

```toml
allow-expect-in-tests = true
```

An exception in production code should describe the invariant that makes it safe. Put this example in `app/src/main.rs`.

```rust
/// Writes one hexadecimal digit for the low four bits.
fn main() {
    #[expect(clippy::expect_used, reason = "The mask restricts the value to 0–15.")]
    let digit = char::from_digit(u32::from(0xabu8 & 0x0f), 16)
        .expect("a four-bit value fits in one hexadecimal digit");
    println!("{digit}");
}
```

Remove the attribute and `cargo clippy` reports `expect_used` at the call. Restore it and the warning is suppressed for that statement. When a later edit removes the cause, `unfulfilled_lint_expectations` asks you to remove the obsolete expectation. Use `#[allow(..., reason = "...")]` when the exception intentionally covers code that may warn only on certain targets or feature combinations. See the [Rust Reference on lint attributes][rust-reference~lint-attributes]↗ and [Clippy configuration options][book~clippy~configuration]↗.

## Gate CI with a Pinned Toolchain {#clippy-ci}

From the workspace root, install the pinned version and create the starter lockfile once. Commit `Cargo.lock` before running the check in CI.

```sh
rustup toolchain install 1.100.0 --component clippy
cargo +1.100.0 generate-lockfile
cargo +1.100.0 clippy --workspace --all-targets --locked -- -D warnings
```

`--all-targets` checks examples, tests, and benches enabled for the selected features and host. Run additional feature or target combinations as needed. Add `--all-features` when those features are meant to coexist. Pinning the toolchain makes new lints part of a deliberate version update. For a library, check the advertised minimum supported Rust version separately.

## How This Book Configures Its Workspaces {#repository-policy}

The maintained book tools use [`tools/Cargo.toml`][rust-howto~tools-lints~repo]↗ for lint levels. Rust denies `unsafe_code` and `missing_docs` there. Clippy warns on `unwrap_used`, `panic`, `map_err_ignore`, `unused_result_ok`, and holding a lock across an await point. It also warns on unreasoned suppressions through `allow_attributes` and `allow_attributes_without_reason`. The list of disallowed methods in [`tools/clippy.toml`][rust-howto~tools-clippy~repo]↗ includes unbounded channel constructors because the tools prefer an explicit capacity.

The [`bk/crates/Cargo.toml`][rust-howto~example-lints~repo]↗ policy allows teaching examples to demonstrate unsafe Rust, numeric casts, and unbounded channels while still warning on `unwrap_used` and `let_underscore_future`. Both workspaces use `all = { level = "warn", priority = -1 }` with selected per-lint overrides. Both Clippy configuration files keep `allow-unwrap-in-tests = false` and set `allow-expect-in-tests = true`. That setting affects `expect_used` when the lint is enabled. The example workspace's [`disallowed-methods` list][rust-howto~example-clippy~repo]↗ only affects packages or scopes that enable the associated lint.

Run the example workspace checks with `just bk lc` from the repository root. The standalone automation guide also describes [checking that workspace members inherit lints][rust-howto~lint-inheritance~repo]↗.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
