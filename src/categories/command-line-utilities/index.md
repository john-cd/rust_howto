# Command-line utilities written in Rust

{{#include index.incl.md}}

[![cat-command-line-utilities][cat-command-line-utilities-badge]][cat-command-line-utilities]

[My terminal became more Rusty Community][blog-rusty-terminal]⮳

[![open-rs-github][open-rs-github-badge]][open-rs-github]

[![starship-github][starship-github-badge]][starship-github]

[![bacon-github][bacon-github-badge]][bacon-github]

## `bat`

`{{i:bat}}` is a fast `cat` clone with syntax highlighting and {{i:Git}} integration.
[![bat-github][bat-github-badge]][bat-github]

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

[`{{i:lsd}}`][lsd-github] is a rewrite of GNU ls with lots of added features like colors, icons, tree-view, additional formatting options.

```sh
apt install lsd
```

## `broot`

[`{{i:broot}}`][broot-website] [![broot-github][broot-github-badge]][broot-github] is a new way to see and navigate directory trees.

## `gping`

Ping, but with a graph [`{{i:gping}}`][gping]

```sh
apt install gping
```

## `exa`

[![exa-github][exa-github-badge]][exa-github]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
