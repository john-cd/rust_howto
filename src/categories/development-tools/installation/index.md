# Installing

{{#include index.incl.md}}

## Rustup

[Rustup][p-rustup]⮳  [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

## Cargo install

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

This command manages Cargo’s local set of installed binary crates. Only packages which have executable{{hi:Executables}} `[[bin]]` or `[[example]]` targets can be installed, and all executables are installed into the installation root’s bin folder. By default only binaries, not examples, are installed.

## Cargo binstall

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

Binstall provides a low-complexity mechanism for installing Rust binaries{{hi:Rust binaries installation}} as an alternative to building from source (via cargo install) or manually downloading packages. This is intended to work with existing CI artifacts and infrastructure, and with minimal overhead for package maintainers.

Binstall works by fetching the crate information from crates.io and searching the linked repository for matching releases and artifacts, falling back to the quickinstall{{hi:quickinstall}} third-party artifact host, to alternate targets as supported, and finally to `cargo install` as a last resort.

[p-rustup]: rustup.md
{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
</div>
