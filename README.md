# Rust How-To

A documentation summary and a compendium of snippets and recipes for the Rust language and ecosystem.

Everything you need for day-to-day Rust coding, all in one place.

## Web site

This book is deployed on <https://john-cd.com/rust_howto>

GitHub repo: <https://john-cd.github.io/rust_howto>

## Install

### VS Code

Clone the repo and open the folder in [VS Code]( https://code.visualstudio.com/ ). It should prompt you to open the code in a Docker container, which installs `mdbook` and couple of its plugins, `just`, `wget` automatically. Make sure you have installed

- [Dev Container extension]( https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers )
- [Docker Desktop]( https://www.docker.com/products/docker-desktop/ ) (or just the Docker engine).

### Other

If you are not using VS Code, install the dev container CLI or simply install on your local machine:

```bash
cargo install mdbook
cargo install mdbook-hide
# optional: cargo install mdbook-keeper --git https://github.com/tfpk/mdbook-keeper.git
```

## Develop

```bash
mdbook serve --open
```

or simply

```bash
just serve
```

Development endpoint: <http://localhost:3000/>

`just` alone lists all commonly used recipes.

To mark a chapter as hidden, add this special Comment anywhere in the Markdown file. It is better to have it at the top of the file for clarity.

```xml
<!--hidden-->
```

## Links

[mdBook]( https://rust-lang.github.io/mdBook/index.html )

[mdbook-hide]( https://github.com/ankitrgadiya/mdbook-hide/ )
