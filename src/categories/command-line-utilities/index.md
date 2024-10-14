# Command-line utilities written in Rust

{{#include index.incl.md}}

[![cat-command-line-utilities][cat-command-line-utilities-badge]][cat-command-line-utilities]

[My terminal became more Rusty Community][blog-rusty-terminal]⮳

[![open-github][c-open-github-badge]][c-open-github]

[![starship-github][c-starship-github-badge]][c-starship-github]

[![bacon-github][c-bacon-github-badge]][c-bacon-github]

## `bat`

`bat`{{hi:bat}} is a fast `cat` clone with syntax highlighting and Git{{hi:Git}} integration.
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

[`lsd`][c-lsd-github]{{hi:lsd}} is a rewrite of GNU ls with lots of added features like colors, icons, tree-view, additional formatting options.

```sh
apt install lsd
```

## `broot`

[`broot`][c-broot-website]{{hi:broot}} [![broot-github][c-broot-github-badge]][c-broot-github] is a new way to see and navigate directory trees.

## `gping`

Ping, but with a graph [`gping`][c-gping-github]{{hi:gping}}

```sh
apt install gping
```

## `exa`

[![exa-github][c-exa-github-badge]][c-exa-github]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO:

</div>
