<!-- TODO Reviewed ??-??-2026 -->

# `templ`

{{#include templ.incl.md}}

## Display `templ` Commands {#templ-help}

Use `templ --help` to list its commands and options. This does not contact
crates.io:

```console
{{#include ../../../../crates/cats/development_tools_documentation/examples/templ/help.sh}}
```

## Generate a Rust by Example Badge Offline {#templ-rbe}

The `rbe` subcommand formats a link and badge for a chapter in the Rust by
Example book. Its output is built from the supplied chapter name, so this
command is deterministic and does not contact a service:

```console
{{#include ../../../../crates/cats/development_tools_documentation/examples/templ/rbe.sh}}
```

## Generate a Known Category Badge Offline {#templ-category-badge}

The `category_badge` CLI command looks up the current crates.io category list,
so it requires network access. When the display name and slug are already
known, the shared template renderer can generate the same badge without
fetching registry metadata:

```rust,editable
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

The examples in this chapter use fixed input for reproducible output. The
registry-backed commands are described here but are not shown as offline
examples.

## Related Topics {#related-topics .skip}

- [`mdbook-utils`][ex~documentation~mdbook-utils]
- [Template Engines][p~template-engine]
- [Rust How-to `templ` source][rust-howto~templ~repo]↗

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

| Recipe                                                                            | Tools                            | Categories                                                                     |
| --------------------------------------------------------------------------------- | -------------------------------- | ------------------------------------------------------------------------------ |
| [Display `templ` Commands][ex~documentation~templ-help]                           | [`templ`][rust-howto~templ~repo] | [![cat~development-tools][cat~development-tools~badge]][cat~development-tools] |
| [Generate a Rust by Example Badge Offline][ex~documentation~templ-rbe]            | [`templ`][rust-howto~templ~repo] | [![cat~development-tools][cat~development-tools~badge]][cat~development-tools] |
| [Generate a Known Category Badge Offline][ex~documentation~templ-category-badge]  | [`templ`][rust-howto~templ~repo] | [![cat~development-tools][cat~development-tools~badge]][cat~development-tools] |
| [Generate Crate and Category References][ex~documentation~templ-crate-references] | [`templ`][rust-howto~templ~repo] | [![cat~development-tools][cat~development-tools~badge]][cat~development-tools] |

<div class="hidden">
  [additional examples from templ tool](https://github.com/john-cd/rust_howto/issues/43)
</div>
