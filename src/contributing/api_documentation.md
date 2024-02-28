## Generate the `docs.rs` Documentation

Use `just doc` to generate the documentation for `docs.rs`.

`cargo doc --open` does not seem to work when running from a {{i:Dev Container}} in {{i:VS Code}}; the script that opens URLs into an external browser (see `$ echo $BROWSER`) does not handle raw HTML. Use `python3 -m http.server 9000` or live server to serve the files instead. See the `doc` recipe in `justfile`.

### Using a Dev Container feature

Alternatively, use the ["Desktop lite" Dev Container feature]( https://github.com/devcontainers/features/tree/main/src/desktop-lite ) to install a light {{i:GUI manager}}. Add the following to `devcontainer.json`:

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

and the following to the {{i:`Dockerfile`}}

```Dockerfile
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && apt-get install -y firefox-esr
```

Optionally `apt-get install xdg-utils` to check that {{i:Firefox}} is the default for `text/html`:

```bash
xdg-mime query default text/html
# or for more details:
XDG_UTILS_DEBUG_LEVEL=2 xdg-mime query default text/html

xdg-settings --list
xdg-settings get default-web-browser
```

Point your browser to `<http://localhost:6080>` and use `vscode` as the password. Open the HTML file of your choice with:

```bash
xdg-open /cargo-target-rust_howto/target/doc/deps/index.html
```

### Other methods to preview the documentation HTML

- Add the target directory e.g. `/cargo-target-rust_howto/target` to the VS Code Explorer view (`File` > `Add Folder to Workspace...`), then right-click the `/cargo-target-rust_howto/target/doc` folder in the VS Code Explorer view and select `Download...` or use VS Code's built-in `Simple Browser` command.
- Or install the `Live Server` or MS `Live Preview` VS Code extensions.

{{#include ../refs/link-refs.md}}
