# Generate the `docs.rs` Documentation {#generate-the-documentation}

{{#include api_documentation.incl.md}}

Use `just doc` to generate the [documentation][p-documentation] for [`docs.rs`][docs-rs]{{hi:docs.rs}}⮳{{hi:docs.rs}}.

`cargo doc --open` does not seem to work when running from a Dev Container{{hi:Dev Container}} in VS Code{{hi:VS code}}; the script that opens URLs into an external browser (see `$ echo $BROWSER`) does not handle raw HTML. Use `python3 -m http.server 9000` or live server to serve the files instead. See the `doc` recipe in [`justfile`][c-just-programmer-manual]{{hi:just}}⮳.

## Using a Dev Container feature {#dev-container-feature}

Alternatively, use the ["Desktop lite" Dev Container feature][desktop-lite-github]{{hi:desktop-lite}}⮳ to install a light GUI manager{{hi:GUI manager}}. Add the following to [`devcontainer.json`][dev-containers-devcontainer.json]⮳{{hi:devcontainer.json}}:

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

and the following to the [`Dockerfile`][dockerfile]{{hi:Dockerfile}}⮳

```Dockerfile
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && apt-get install -y firefox-esr
```

Optionally `apt-get install xdg-utils` to check that Firefox{{hi:Firefox}} is the default for `text/html`:

```bash
xdg-mime query default text/html
# or for more details:
XDG_UTILS_DEBUG_LEVEL=2 xdg-mime query default text/html

xdg-settings --list
xdg-settings get default-web-browser
```

Point your browser to [http://localhost:6080][locahost:6080] and use [`vscode`][vscode-website]⮳{{hi:vscode}} as the password. Open the HTML file of your choice with:

```bash
xdg-open /cargo-target-rust_howto/target/doc/deps/index.html
```

### Other methods to preview the documentation HTML {#other-methods-to-preview-the-documentation-html}

- Add the target directory e.g. `/cargo-target-rust_howto/target` to the VS Code Explorer view (`File` > `Add Folder to Workspace...`), then right-click the `/cargo-target-rust_howto/target/doc` folder in the VS Code Explorer view and select `Download...` or use VS Code's built-in `Simple Browser` command.
- Or install the `Live Server` or MS `Live Preview` VS Code extensions.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[P1 review](https://github.com/john-cd/rust_howto/issues/980)

</div>
