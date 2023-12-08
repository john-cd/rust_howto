# Web site

This book is deployed on <https://john-cd.com/rust_howto>

repo: <https://john-cd.github.io/rust_howto>


## Install

VS Code should prompt you to open the code in a Dev Container, which installs `just`, `wget`, `mdbook` and its plugins automatically.
If you are not using VS Code, install the dev container CLI or simply install on your local machine:

```bash
cargo install mdbook
cargo install mdbook-hide
# in the future: cargo install mdbook-keeper
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

```
<!--hidden-->
```

## Links

[mdBook]( https://rust-lang.github.io/mdBook/index.html )

[mdbook-hide]( https://github.com/ankitrgadiya/mdbook-hide/ )
