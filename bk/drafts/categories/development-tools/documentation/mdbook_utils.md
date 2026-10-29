<!-- TODO Reviewed ??-??-2026 -->

# `mdbook-utils`

{{#include mdbook_utils.incl.md}}

## Review Links and References {#review-links-and-references}

[`mdbook-utils`][mdbook-utils~repo~badge][mdbook-utils~repo]↗ is a companion
command-line tool for maintaining large Markdown books. Its link commands
write reports without rewriting link syntax in place. Use `duplicate-links`
and `broken-links` to review sources; `write-all` and `write-inline` export
parsed links. The `refdefs write` command exports existing definitions, while
`refdefs generate` prepares GitHub badge references or references from a
book's Cargo dependencies.

The following local-book commands use the source directory and write their
reports to explicit files:

```console
{{#include ../../../../crates/cats/development_tools_documentation/examples/mdbook_utils/link_reports.sh}}
```

`links write-inline` also reports external inline links and autolinks. Choose
`--link-format reference`, `inline`, or `autolink`; it does not change the
source files. Link-like text in code is preserved.

The same command can be driven from Rust, for example in a build or test
helper. This example writes a one-page book to a temporary directory and
reports its undefined reference; its test is ignored unless `mdbook-utils` is
installed.

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/documentation/mdbook_utils_broken_links.rs:example}}
```

## Keep Code Examples in Separate Files {#separate-code-examples}

`markdown extract-code-examples` copies fenced Rust blocks to files under a
destination directory. `replace-code-examples-by-includes` can then replace
those blocks with mdBook include directives. The extraction command reports a
conflict instead of overwriting a changed output file. The replacement command
modifies Markdown after confirmation, so review a diff before accepting it.

```console
{{#include ../../../../crates/cats/development_tools_documentation/examples/mdbook_utils/code_examples.sh}}
```

Use `replace-includes-by-contents` to expand includes for tools that do not
understand mdBook directives. It leaves `*-refs.md` includes intact. The
`identify-files-not-in-summary` and `identify-unused-rs-examples` commands help
find chapters missing from `SUMMARY.md` and Rust examples no page uses.
`remove-includes` replaces remaining include directives in an expanded
Markdown build, for example before passing that output to a renderer that does
not understand mdBook helpers.

```console
{{#include ../../../../crates/cats/development_tools_documentation/examples/mdbook_utils/find_unreferenced_files.sh}}
```

## Generate Indexes and a Sitemap {#generate-indexes-and-sitemap}

`markdown generate-crates` and `generate-categories` build Markdown indexes
from the book's manifests. `sitemap` reads the chapter paths from `SUMMARY.md`
and includes pages only when their rendered HTML exists. Write the sitemap
inside the HTML renderer's output directory and pass the site's real base URL.
Use `debug` to write the parser events for a source tree when diagnosing a
Markdown parsing issue; it also operates entirely on local files.

```console
{{#include ../../../../crates/cats/development_tools_documentation/examples/mdbook_utils/indexes_and_sitemap.sh}}
```

For installation, configuration precedence, and every command option, see the
[`mdbook-utils` user guide][mdbook-utils~guide]↗. Use
`mdbook-utils <command> <subcommand> --help` to inspect the installed version's
options. The tool is maintained in its own repository; its source and test
suite are separate from this book's examples.

## Related Topics {#related-topics .skip}

- [Create and Build a Book][ex~documentation~mdbook]
- [Generate Markdown Fragments with `templ`][ex~documentation~templ]
- [`book_tooling`][book_tooling~repo]↗ handles in-place inline-link rewrites,
  reference cleanup, and HTTP link checking.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

| Recipe                                                                | Tools                             | Categories                                                                     |
| --------------------------------------------------------------------- | --------------------------------- | ------------------------------------------------------------------------------ |
| [Review Links and References][ex~documentation~mdbook-utils]          | [mdbook-utils][mdbook-utils~repo] | [![cat~development-tools][cat~development-tools~badge]][cat~development-tools] |
| [Keep Code Examples in Separate Files][ex~documentation~mdbook-utils] | [mdbook-utils][mdbook-utils~repo] | [![cat~development-tools][cat~development-tools~badge]][cat~development-tools] |
| [Generate Indexes and a Sitemap][ex~documentation~mdbook-utils]       | [mdbook-utils][mdbook-utils~repo] | [![cat~development-tools][cat~development-tools~badge]][cat~development-tools] |

<div class="hidden">
  [additional examples from mdbook-utils](https://github.com/john-cd/rust_howto/issues/41)
</div>
