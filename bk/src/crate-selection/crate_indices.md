<!-- TODO Reviewed ??-??-2026 -->

# Generate Crate Indices Offline

[![crate_indices][crate-indices~repo~badge]][crate-indices~repo]

The repository's `crate_indices` helper binary uses `tool_lib` to generate book indices. Its local APIs work from Cargo manifests, checked-in category metadata, and the published example index, so these operations can be exercised with temporary fixtures and do not need crates.io access.

| Capability                     | `tool_lib` API                                         | Input                                                                   |
| ------------------------------ | ------------------------------------------------------ | ----------------------------------------------------------------------- |
| List example dependencies      | `get_dependencies`                                     | Local `Cargo.toml` files                                                |
| Find published recipes         | `collect_published_recipe_links`                       | The mdBook example index, its includes, and local reference definitions |
| Render a category index        | `create_category_page_from_metadata_with_recipe_links` | Crate names, a TOML category snapshot, and collected recipe links       |
| Render an alphabetical section | `create_alphabetical_crate_page_section`               | Crate names and collected recipe links                                  |

The online crate-information commands are separate; the example below demonstrates only the local operations.

## List local dependencies

```rust,editable
{{#include ../../crates/cats/development_tools/examples/crate_indices/dependencies.rs:example}}
```

## Collect published recipes

```rust,editable
{{#include ../../crates/cats/development_tools/examples/crate_indices/published_recipes.rs:example}}
```

## Generate a category page

```rust,editable
{{#include ../../crates/cats/development_tools/examples/crate_indices/category_page.rs:example}}
```

## Generate an alphabetical section

```rust,editable
{{#include ../../crates/cats/development_tools/examples/crate_indices/alphabetical_page.rs:example}}
```

Each module has a behavior test that uses the shared local fixtures. Run them from `bk/crates` with `cargo test -p development_tools --example crate_indices --locked`. These examples make no external HTTP requests.

## Related Topics {#related-topics .skip}

- [Crate Selection](index.md).
- [Crate indices][crate-indices~repo].
- [Cargo manifests and dependencies][ex~code-organization~dependencies].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
