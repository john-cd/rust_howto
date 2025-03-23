# Temporary Files and Directories

{{#include tempfile.incl.md}}

## Create Temporary Files or Temporary Directories {#temporary-files-or-directories}

[![tempfile-website][c-tempfile-website-badge]][c-tempfile-website] [![tempfile][c-tempfile-badge]][c-tempfile] [![tempfile-crates.io][c-tempfile-crates.io-badge]][c-tempfile-crates.io] [![tempfile-github][c-tempfile-github-badge]][c-tempfile-github] [![tempfile-lib.rs][c-tempfile-lib.rs-badge]][c-tempfile-lib.rs]{{hi:tempfile}}{{hi:Filesystem}}{{hi:Tempfile}}

[`tempfile`][c-tempfile]⮳{{hi:tempfile}} supports both temporary files and temporary directories.

```rust,editable
{{#include ../../../crates/cats/filesystem/tests/tempfile/tempfile.rs:example}}
```

### Other Options

- The `tempdir` crate is being merged into `tempfile`.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[tempfile: write NOW](https://github.com/john-cd/rust_howto/issues/361)
</div>
