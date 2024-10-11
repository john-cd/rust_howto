# Command-line utilities written in Rust

{{#include index.incl.md}}

[![cat-command-line-utilities][cat-command-line-utilities-badge]][cat-command-line-utilities]

[My terminal became more Rusty Community][blog-rusty-terminal]⮳

[![open-rs-github][c-open-rs-github-badge]][c-open-rs-github]

[![starship-github][c-starship-github-badge]][c-starship-github]

[![bacon-github][c-bacon-github-badge]][c-bacon-github]

## `bat`

`{{i:bat}}` is a fast `cat` clone with syntax highlighting and {{i:Git}} integration.
[![bat-github][c-bat-github-badge]][c-bat-github]

```sh
bat README.md
# Display multiple files at once
bat src/*.rs
# Read from stdin, determine the syntax automatically
curl -s https://sh.rustup.rs | bat
# Show and highlight non-printable characters:
bat -A /etc/hosts
```

## `lsd`

[`{{i:lsd}}`][c-lsd-github] is a rewrite of GNU ls with lots of added features like colors, icons, tree-view, additional formatting options.

```sh
apt install lsd
```

## `broot`

[`{{i:broot}}`][c-broot-website] [![broot-github][c-broot-github-badge]][c-broot-github] is a new way to see and navigate directory trees.

## `gping`

Ping, but with a graph [`{{i:gping}}`][c-gping-github]

```sh
apt install gping
```

## `exa`

[![exa-github][c-exa-github-badge]][c-exa-github]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
