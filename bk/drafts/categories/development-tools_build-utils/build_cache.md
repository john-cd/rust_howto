# Build Caches

{{#include build_cache.incl.md}}

## Use `sccache` {#sccache}

[![sccache][c-sccache-badge]][c-sccache] [![sccache-crates.io][c-sccache-crates.io-badge]][c-sccache-crates.io] [![sccache-github][c-sccache-github-badge]][c-sccache-github] [![sccache-lib.rs][c-sccache-lib.rs-badge]][c-sccache-lib.rs]{{hi:sccache}}{{hi:Ccache}} [![cat-command-line-utilities][cat-command-line-utilities-badge]][cat-command-line-utilities]{{hi:Command line utilities}} [![cat-development-tools::build-utils][cat-development-tools::build-utils-badge]][cat-development-tools::build-utils]{{hi:Build Utils}}

Sccache is a ccache-like tool. It is used as a compiler wrapper and avoids compilation when possible. Sccache has the capability to utilize [caching][p-caching] in remote storage environments, including various cloud storage options, or alternatively, in local storage.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1167)
</div>
