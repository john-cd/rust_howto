# Installing

{{#include install.incl.md}}

## `rustup` {#rustup}

[Rustup][p-rustup]⮳ [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}

Rustup installs, manages, and upgrades versions of rustc, cargo, clippy, rustfmt, etc.

## `cargo install` {#cargo-install}

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}

This command manages `cargo`’s local set of installed binary crates. Only packages which have executable{{hi:Executables}} `[[bin]]` or `[[example]]` targets can be installed, and all executables are installed into the installation root’s bin folder. By default only binaries, not examples, are installed.

## `cargo binstall` {#cargo-binstall}

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}

`cargo binstall` provides a low-complexity mechanism for installing Rust binaries{{hi:Rust binaries installation}} as an alternative to building from source (via cargo install) or manually downloading packages. This is intended to work with existing CI artifacts and infrastructure, and with minimal overhead for package maintainers.

`cargo binstall` works by fetching the crate information from crates.io and searching the linked repository for matching releases and artifacts, falling back to the quickinstall{{hi:quickinstall}} third-party artifact host, to alternate targets as supported, and finally to `cargo install` as a last resort.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
Add link for `cargo binstall`
</div>
