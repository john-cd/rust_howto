# File listing and display

## `lsd` {#lsd`}

[`lsd`][c-lsd-github]{{hi:lsd}} is a rewrite of GNU ls with lots of added features like colors, icons, tree-view, additional formatting options.

```sh
apt install lsd
```

## `exa` {#exa}

[![exa-github][c-exa-github-badge]][c-exa-github]{{hi:exa}}

exa is a modern replacement for ls.

## `broot` {#broot}

[`broot`][c-broot-website]{{hi:broot}} [![broot-github][c-broot-github-badge]][c-broot-github] is a new way to see and navigate directory trees.

## `bat` {#bat}

`bat`{{hi:bat}} is a fast `cat` clone with syntax highlighting and Git{{hi:Git}} integration.
[![bat-github][c-bat-github-badge]][c-bat-github]{{hi:bat}}

```sh
bat README.md
# Display multiple files at once
bat src/*.rs
# Read from stdin, determine the syntax automatically
curl -s https://sh.rustup.rs | bat
# Show and highlight non-printable characters:
bat -A /etc/hosts
```

## open {#open}

[![open][c-open-badge]][c-open]{{hi:open}}
[![open-crates.io][c-open-crates.io-badge]][c-open-crates.io]
[![open-github][c-open-github-badge]][c-open-github]
[![open-lib.rs][c-open-lib.rs-badge]][c-open-lib.rs]

Opens a path or URL using the program configured on the system.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: organize
</div>
