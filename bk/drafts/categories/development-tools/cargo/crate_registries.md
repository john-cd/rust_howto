<!-- TODO Reviewed ??-??-2026 -->
# Crate Registries

{{#include crate_registries.incl.md}}

Cargo resolves package dependencies from registries. A registry stores published crate versions and their metadata; it is separate from source browsing sites such as [`lib.rs`][lib.rs~website]↗, which indexes crates but is not a Cargo registry.

## Cargo Registries {#crate-registries}

### Use the Default Registry

[`crates.io`][crates.io~website]↗ is the default public registry. Cargo also supports Git-based and sparse HTTP registry protocols. New registry configuration should generally use the sparse protocol, which avoids cloning an index repository.

## Configure an Alternate Registry {#configure-an-alternate-registry}

Add a named registry to `.cargo/config.toml` (or a Cargo home configuration file):

```toml
[registries.company]
index = "sparse+https://packages.example.com/index/"
```

Then select that registry for a dependency in `Cargo.toml`:

```toml
[dependencies]
internal-library = { version = "1", registry = "company" }
```

The registry name is a local alias for its index URL. The registry must also be configured for any developer or CI environment that builds the package. Credentials belong in Cargo's credential provider or another supported secret store; do not commit tokens to a manifest or repository configuration.

Publish to the selected registry with `cargo publish --registry company`. The registry must be configured, and authentication must be available when publishing. crates.io does not accept packages whose dependencies come from another registry.

## Registry Configuration and Source Replacement

Cargo's `[registries]` configuration names additional registries for dependency selection and publishing. Source replacement is a different mechanism: it substitutes one source for another, commonly to use a local vendor directory or mirror while preserving the package source identity. Use the registry mechanism to intentionally depend on packages from a separate registry, and source replacement for mirrors or vendored copies.

See the [Cargo registry reference][book~cargo~registries]↗, [configuration reference][book~cargo~config]↗, and [registry authentication guide][book~cargo~registry-authentication]↗ for the available protocols and credential options.

## Access crates.io Metadata with `crates-io` {#crates-io}

[![crates-io][c~crates-io~docs~badge]][c~crates-io~docs] [![crates-io~crates.io][c~crates-io~crates.io~badge]][c~crates-io~crates.io] [![crates-io~repo][c~crates-io~repo~badge]][c~crates-io~repo] [![crates-io~lib.rs][c~crates-io~lib.rs~badge]][c~crates-io~lib.rs]{{hi:crates-io}}

The `crates-io` crate provides helpers for querying crates.io metadata through its web API. It is an API client, not a way to configure Cargo's package source.

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/crate_registries/crates-io.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[write; expand](https://github.com/john-cd/rust_howto/issues/294)
</div>
