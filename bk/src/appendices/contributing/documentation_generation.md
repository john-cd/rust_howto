<!-- TODO Reviewed ??-??-2026 -->

# Documentation Generation

{{#include documentation_generation.incl.md}}

## Generate Markdown with `templ` {#templ}

The repository's [`templ` helper][templ~source]↗ generates Markdown fragments for the book. Build the helper with `just release` from `tools/`, then run the examples from the repository root. The help and category-badge commands use local templates and reference data; `rbe` formats a supplied Rust by Example chapter name.

```bash
bash bk/crates/cats/development_tools_documentation/examples/templ/help.sh
```

```bash
bash bk/crates/cats/development_tools_documentation/examples/templ/category_badge.sh
```

```bash
bash bk/crates/cats/development_tools_documentation/examples/templ/rbe.sh
```

Commands that fetch crate metadata depend on crates.io and are omitted from these offline examples.

## Manage mdBook Sources with `mdbook-utils` {#mdbook-utils}

[![mdbook-utils][mdbook-utils~docs~badge]][mdbook-utils~docs] [![mdbook-utils~repo][mdbook-utils~repo~badge]][mdbook-utils~repo]

[`mdbook-utils`][mdbook-utils~repo]↗ manages reference definitions, links, code examples, and sitemaps. See the [user guide][mdbook-utils~user-guide] for installation and command options. Run the examples from `bk/`; the link and reference reports are local and repeatable. The online `broken-links` command is omitted from the script because its results depend on external HTTP services.

```bash
bash crates/cats/development_tools_documentation/examples/mdbook_utils/link_reports.sh
```

Build the book before generating a sitemap. The fixed base URL keeps the output stable:

```bash
bash crates/cats/development_tools_documentation/examples/mdbook_utils/indexes_and_sitemap.sh
```

Find pages missing from the book summary and Rust examples that are not included:

```bash
bash crates/cats/development_tools_documentation/examples/mdbook_utils/find_unreferenced_files.sh
```

Extract code examples and resolve includes in a scratch `drafts/` directory before changing book sources:

```bash
bash crates/cats/development_tools_documentation/examples/mdbook_utils/code_examples.sh
```

Use `just docs docall` to generate the code [[documentation]] in the [`docs.rs`][docs.rs~website]↗{{hi:docs.rs}} format.

Note that [`cargo doc --open`][book~cargo~cargo-doc]↗{{hi:cargo doc}} does not seem to work when running from a Dev Container{{hi:Dev Container}} in VS Code{{hi:VS Code}}; the script that opens URLs into an external browser (see `$ echo $BROWSER`) does not handle raw HTML. Use `python3 -m http.server 9000` (or live server) to serve the files instead.

Read the `justfile`{{hi:justfile}} or the [just module][c~just~programmer-manual]↗ in `bk/scripts/docs`{{hi:just}} for more details.

## Use a "Dev Container" Feature {#dev-container-feature}

Alternatively, use the ["Desktop lite" Dev Container feature][desktop-lite~repo]↗{{hi:desktop-lite}} to install a light GUI manager{{hi:GUI manager}}. Add the following to [`devcontainer.json`][dev-containers-devcontainer.json]↗{{hi:devcontainer.json}}:

```json
"features": {
    "ghcr.io/devcontainers/features/desktop-lite:1": {}
},
"forwardPorts": [
    6080
],
"portsAttributes": {
    "6080": {
        "label": "desktop"
    }
},
```

and the following to the [`Dockerfile`][dockerfile~website]↗{{hi:Dockerfile}}.

```Dockerfile
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && apt-get install -y firefox-esr
```

Optionally [`apt-get install xdg-utils`][xdg-utils~website]↗{{hi:xdg-utils}} to check that Firefox{{hi:Firefox}} is the default for `text/html`:

```bash
xdg-mime query default text/html
# or for more details:
XDG_UTILS_DEBUG_LEVEL=2 xdg-mime query default text/html

xdg-settings --list
xdg-settings get default-web-browser
```

Point your browser to [http://localhost:6080][localhost:6080~website]↗ and use [`vscode`][vscode~website]↗{{hi:VS Code}} as the password. Open the HTML file of your choice with:

```bash
xdg-open /code/target/bk/doc/deps/index.html
```

### Other Methods to Preview the Documentation HTML {#other-methods-to-preview-the-documentation-html}

- Right-click the `/code/target/bk/doc` folder in the VS Code Explorer view and select `Download...` or use VS Code's built-in `Simple Browser` command.
- Or install the [`Live Server`][vscode~LiveServer~website]↗{{hi:Live Server}} or MS `Live Preview` VS Code extensions.

## Related Topics {#related-topics .skip}

- [[book_editing_and_example_code_development | Book Editing and Example Code Development]].
- [[dev_container_and_docker | Dev Container and Docker]].
- [[development_environment_setup | Development Environment Setup]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  accuracy review: contributing guide
</div>
