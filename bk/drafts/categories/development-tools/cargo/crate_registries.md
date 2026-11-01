<!-- TODO Reviewed ??-??-2026 -->

# Crate Registries

{{#include crate_registries.incl.md}}

Cargo resolves package dependencies from registries. A registry stores published crate versions and their metadata; it is separate from source browsing sites such as [`lib.rs`][lib.rs~website]↗, which indexes crates but is not a Cargo registry.

## Cargo Registries {#crate-registries}

### Use the Default Registry

[`crates.io`][crates.io~website]↗ is the default public registry. Cargo also supports Git-based and sparse HTTP registry protocols. New registry configuration should generally use the sparse protocol, which avoids cloning an index repository.

## Use an Alternative Registry {#use-an-alternative-registry}

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

## Publish to a Private Registry {#publish-to-a-private-registry}

Private registries let a team distribute internal crates or control access to packages. Examples include the self-hosted [Kellnr][kellnr~repo]↗ registry and hosted Cargo registries from [Cloudsmith][cloudsmith~cargo]↗ and [JFrog Artifactory][jfrog~cargo]↗. The provider supplies the index URL and authentication setup; configure those in Cargo and a credential provider appropriate to the environment.

For example, a team registry might be configured like this:

```toml
[registries.company]
index = "sparse+https://packages.example.com/index/"
```

Select it in a package manifest with `registry = "company"`, then publish with `cargo publish --registry company`. Use `cargo login --registry company` only when the registry's authentication method supports Cargo's token provider; some providers supply a credential helper instead. Keep secrets out of checked-in files.

## Vendor or Mirror Dependencies {#vendor-or-mirror-dependencies}

Vendoring places the source for registry dependencies in a project directory, which can support offline builds and make dependency sources available to a controlled build environment. From the package or workspace root, run:

```console
cargo vendor vendor
```

Cargo prints the source replacement configuration needed to use that directory. A typical `.cargo/config.toml` configuration is:

```toml
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
```

Source replacement is for an exact copy or mirror of a source. It does not configure a separate private registry; use `[registries]` and a dependency's `registry` key for that. Commit the generated `vendor/` directory only when the project needs a checked-in dependency mirror, and refresh it with `cargo vendor` when dependencies change.

## Registry Configuration and Source Replacement

Cargo's `[registries]` configuration names additional registries for dependency selection and publishing. Source replacement is a different mechanism: it substitutes one source for another, commonly to use a local vendor directory or mirror while preserving the package source identity. Use the registry mechanism to intentionally depend on packages from a separate registry, and source replacement for mirrors or vendored copies.

See the [Cargo registry reference][book~cargo~registries]↗, [configuration reference][book~cargo~config]↗, [source replacement reference][book~cargo~source-replacement]↗, and [registry authentication guide][book~cargo~registry-authentication]↗ for the available protocols and credential options. For publishing to the public registry, see [Publication to `crates.io`][p~publication-to-crates-io].

## Access crates.io Metadata with `crates-io` {#crates-io}

{{#badges crates-io}}

The `crates-io` crate provides helpers for querying crates.io metadata through its web API. It is an API client, not a way to configure Cargo's package source.

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/crate_registries/crates_io.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Cargo configuration][book~cargo~config]↗.
- [Publish to `crates.io`][p~publication-to-crates-io].

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
