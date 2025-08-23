# `docs.rs` Documentation Generation

{{#include api_documentation.incl.md}}

Use `just docs docall` to generate the code [documentation][p~documentation] in the [`docs.rs`][docs.rs~website]↗{{hi:docs.rs}}{{hi:docs.rs}} format.

Note that [`cargo doc --open`][book~cargo~cargo-doc]↗{{hi:cargo doc}} does not seem to work when running from a Dev Container{{hi:Dev Container}} in VS Code{{hi:VS Code}}; the script that opens URLs into an external browser (see `$ echo $BROWSER`) does not handle raw HTML. Use `python3 -m http.server 9000` (or live server) to serve the files instead.

Read the `justfile`{{hi:justfile}} or the [just module][c~just~programmer-manual]↗ in `bk/scripts/docs`{{hi:just}} for more details.

## Using a Dev Container Feature {#dev-container-feature}

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

Point your browser to [http://localhost:6080][locahost:6080~website]↗ and use [`vscode`][vscode~website]↗{{hi:VS Code}} as the password. Open the HTML file of your choice with:

```bash
xdg-open /code/target/bk/doc/deps/index.html
```

### Other Methods to Preview the Documentation HTML {#other-methods-to-preview-the-documentation-html}

- Right-click the `/code/target/bk/doc` folder in the VS Code Explorer view and select `Download...` or use VS Code's built-in `Simple Browser` command.
- Or install the [`Live Server`][vscode~LiveServer~website]↗{{hi:Live Server}} or MS `Live Preview` VS Code extensions.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/980)
</div>
