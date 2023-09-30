

# Web site

<https://john-cd.com/rust_howto>

<https://john-cd.github.io/rust_howto>

## Install

```bash
cargo install mdbook
cargo install mdbook-hide
```

## Develop

```bash
mdbook serve --open
```

Development endpoint: <http://localhost:3000/>

To mark a chapter as hidden, add this special Comment anywhere in the Markdown file. It is better to have it at the top of the file for clarity.

```
<!--hidden-->
```

## Links

[mdBook]( https://rust-lang.github.io/mdBook/index.html )

[mdbook-hide]( https://github.com/ankitrgadiya/mdbook-hide/ )
