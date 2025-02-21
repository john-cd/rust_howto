# Build Time Tooling

[![cat-development-tools::build-utils][cat-development-tools::build-utils-badge]][cat-development-tools::build-utils]{{hi:Development tools::Build Utils}}

Utilities for build scripts and other build time steps.

{{#include build_utils.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[development-tools_build-utils/index: add (P2)](https://github.com/john-cd/rust_howto/issues/306)

## Use `sccache`

[![sccache][c-sccache-badge]][c-sccache] [![sccache-crates.io][c-sccache-crates.io-badge]][c-sccache-crates.io] [![sccache-github][c-sccache-github-badge]][c-sccache-github] [![sccache-lib.rs][c-sccache-lib.rs-badge]][c-sccache-lib.rs]{{hi:sccache}}{{hi:Ccache}} [![cat-command-line-utilities][cat-command-line-utilities-badge]][cat-command-line-utilities]{{hi:Command line utilities}} [![cat-development-tools::build-utils][cat-development-tools::build-utils-badge]][cat-development-tools::build-utils]{{hi:Build Utils}}

Sccache is a ccache-like tool. It is used as a compiler wrapper and avoids compilation when possible. Sccache has the capability to utilize caching in remote storage environments, including various cloud storage options, or alternatively, in local storage.

Task Runners: `xtask` (for managing complex build tasks, often used for CI/CD, testing, etc.)
Build Script Helpers: `cc` (for compiling C/C++ code within build scripts), `pkg-config` (for finding system libraries)
Cross-Compilation Tools: `cross` (simplifies cross-compilation using Docker)
Dependency Management: `cargo` (though primarily a package manager, it also handles build dependencies)
Code Generation: (often done with procedural macros or build scripts; no single "build util" crate for this)
Packaging: `cargo-deb`, `cargo-rpm`, `create-dmg` (for creating distribution packages)
Documentation Generation: `cargo doc` (though primarily for documentation, it's part of the build process)
Testing: `cargo test` (part of the build process)
Benchmarking: `cargo bench` (also part of the build/verification process)
</div>
