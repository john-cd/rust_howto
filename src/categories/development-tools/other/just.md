# Just

[https://just.systems/][just-website]⮳  [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

`{{i:just}}` is a {{i:command runner}} / Make replacement.

[Just Programmer's Manual][just-programmer-manual]⮳

## Installation in a dev container

```Dockerfile
FROM mcr.microsoft.com/devcontainers/base:bullseye
# or perhaps mcr.microsoft.com/devcontainers/rust:bullseye if you want rust & cargo

SHELL ["bash", "-c"]

# Prerequisites to install Just: https://just.systems/man/en/chapter_4.html
RUN <<EOF
  wget -qO - 'https://proget.makedeb.org/debian-feeds/prebuilt-mpr.pub' | gpg --dearmor | sudo tee /usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg 1> /dev/null
  echo "deb [arch=all,$(dpkg --print-architecture) signed-by=/usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg] https://proget.makedeb.org prebuilt-mpr $(lsb_release -cs)" | sudo tee /etc/apt/sources.list.d/prebuilt-mpr.list
  sudo apt update
EOF

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install just \
    && apt-get autoremove -y && apt-get clean -y
```

For Alpine, use [`{{i:apk}}`][apk-website]⮳:

```dockerfile
## Just: https://just.systems/man/en/chapter_1.html
RUN apk add just
```

## Example `justfile`

Place it in the root folder of your project. Run [`{{i:just}}`][just-website] to see a list of recipes. Run `just <recipe>` to execute the corresponding recipe.

```makefile
# Load a .env file, if present.
set dotenv-load

default:
  @just --list --unsorted

# Check a local package and all of its dependencies for errors
check:
  @cargo check

# Compile a local package and all of its dependencies
build: check
  @cargo build

# Run a binary or example of the local packagels
run: check
  @cargo run

system-info:
  @echo "This is an {{arch()}} machine".

# Shebang script example
foo:
  #!/usr/bin/env bash
  set -euxo pipefail
  hello='Yo'
  echo "$hello from Bash!"
```

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
