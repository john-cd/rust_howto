<!-- TODO Reviewed ??-??-2026. -->

# `templ`

{{#include templ.incl.md}}

## Display `templ` Commands {#templ-help}

Use `templ --help` to list its commands and options. This does not contact
crates.io:

```console
{{#include ../../../../crates/cats/development_tools/documentation/examples/templ/help.sh}}
```

## Generate a Rust by Example Badge Offline {#templ-rbe}

The `rbe` subcommand formats a link and badge for a chapter in the Rust by
Example book. Its output is built from the supplied chapter name, so this
command is deterministic and does not contact a service:

```console
{{#include ../../../../crates/cats/development_tools/documentation/examples/templ/rbe.sh}}
```

## Generate a Known Category Badge Offline {#templ-category-badge}

The `category_badge` CLI command looks up the current crates.io category list,
so it requires network access. When the display name and slug are already
known, the shared template renderer can generate the same badge without
fetching registry metadata:

Test this Cargo-backed example from `bk/crates` with `cargo test --locked -p development_tools --example templ_category_badge`.

```rust,editable,noplayground
{{#include ../../../../crates/cats/development_tools/examples/documentation/templ_category_badge.rs:example}}
```

## Generate Crate and Category References {#templ-crate-references}

The `badge` subcommand creates a crate badge block with reference definitions;
`--add` merges the definitions into a local reference file. `info` prints crate
metadata, while `categories_badges_for_crate` prints category badges for a
crate. These commands query crates.io, so their result depends on current
registry metadata and they require network access.

`category_badge` finds category names that match the supplied search term and
creates badges for them by retrieving the current category list from crates.io.
It therefore requires network access, as do the other registry-backed commands.

The offline examples use fixed input. The registry-backed commands below query
crates.io and return current metadata.

Run the registry-backed commands with a crate name. They query crates.io and
their output reflects current registry metadata:

```console
templ badge serde
templ categories_badges_for_crate serde
templ info serde
```

`templ badge` can merge reference definitions into a file with `--add PATH`.
That option edits the selected file; review its diff before keeping the changes.

## Related Topics {#related-topics .skip}

- [`mdbook-utils`][ex~documentation~mdbook-utils]
- [Template Engines][p~template-engine]
- [Rust How-to `templ` source][rust-howto~templ~repo]↗

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

| Recipe | Tools | Categories |
| --- | --- | --- |
| [Display `templ` Commands][ex~documentation~templ-help] | [`templ`][rust-howto~templ~repo] | {{#categories development-tools="Development tools"}} |
| [Generate a Rust by Example Badge Offline][ex~documentation~templ-rbe] | [`templ`][rust-howto~templ~repo] | {{#categories development-tools="Development tools"}} |
| [Generate a Known Category Badge Offline][ex~documentation~templ-category-badge] | [`templ`][rust-howto~templ~repo] | {{#categories development-tools="Development tools"}} |
| [Generate Crate and Category References][ex~documentation~templ-crate-references] | [`templ`][rust-howto~templ~repo] | {{#categories development-tools="Development tools"}} |
