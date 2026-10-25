<!-- TODO Reviewed ??-??-2026 -->

# `templ`

{{#include templ.incl.md}}

## Generate a Rust by Example Badge Offline {#templ-rbe}

The `rbe` subcommand formats a link and badge for a chapter in the Rust by
Example book. Its output is built from the supplied chapter name, so this
command is deterministic and does not contact a service:

```console
{{#include ../../../../crates/cats/development_tools_documentation/examples/templ/rbe.sh}}
```

## Generate Crate and Category References {#templ-crate-references}

The `badge` subcommand creates a crate badge block with reference definitions;
`--add` merges the definitions into a local reference file. `info` prints crate
metadata, while `categories_badges_for_crate` prints category badges for a
crate. These commands query crates.io, so their result depends on current
registry metadata and they require network access.

`category_badge` finds category names that match the supplied search term and
creates badges for them. It retrieves the current category list from the
crates.io repository, so it also requires network access.

Inspect the installed tool's options before generating files:

```console
{{#include ../../../../crates/cats/development_tools_documentation/examples/templ/help.sh}}
```

The command inventory below is based on `templ --help`. Only the `rbe`
subcommand is suitable for an offline, deterministic example without adding a
fixture or a local-data option to the tool.

## Related Topics {#related-topics .skip}

- [Template Engines][p~template-engine]
- [Rust How-to `templ` source][rust-howto~templ~repo]↗

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

| Recipe                                                             | Tools                            | Categories                                                                     |
| ------------------------------------------------------------------ | -------------------------------- | ------------------------------------------------------------------------------ |
| [Generate a Rust by Example Badge Offline][ex~documentation~templ] | [`templ`][rust-howto~templ~repo] | [![cat~development-tools][cat~development-tools~badge]][cat~development-tools] |
| [Generate Crate and Category References][ex~documentation~templ]   | [`templ`][rust-howto~templ~repo] | [![cat~development-tools][cat~development-tools~badge]][cat~development-tools] |

<div class="hidden">
  [additional examples from templ tool](https://github.com/john-cd/rust_howto/issues/43)
</div>
