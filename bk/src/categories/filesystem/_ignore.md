# Walk the filesystem

{{#include ignore.incl.md}}

## Walk the filesystem while respecting ignore files {#walk-the-filesystem-while-respecting-ignore-files}

[![ignore][c-ignore-badge]][c-ignore] [![ignore-crates.io][c-ignore-crates.io-badge]][c-ignore-crates.io] [![ignore-github][c-ignore-github-badge]][c-ignore-github] [![ignore-lib.rs][c-ignore-lib.rs-badge]][c-ignore-lib.rs]{{hi:ignore}}{{hi:File}}{{hi:Gitignore}}{{hi:Glob}}{{hi:ignore}}{{hi:Pattern}}

`ignore` is a library for efficiently matching ignore files such as `.gitignore` against file paths.

Recursive filesystem walking that respects ignore files (like .gitignore)

```rust,editable
{{#include ../../../crates/ex/cats/filesystem/tests/directory_traversal/ignore.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ignore: write (P1)](https://github.com/john-cd/rust_howto/issues/360)

</div>
