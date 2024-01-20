## Download a file to a temporary directory

[![reqwest-badge]][reqwest] [![tempdir-badge]][tempdir] [![cat-net-badge]][cat-net] [![cat-filesystem-badge]][cat-filesystem]

Creates a temporary directory with [`tempfile::Builder`] and downloads
a file over HTTP using [`reqwest::get`] asynchronously.

Creates a target [`File`] with name obtained from [`Response::url`] within
[`tempdir()`] and copies downloaded data into it with [`io::copy`].
The temporary directory is automatically removed on program exit.

```rust,editable,no_run
{#include ../../../deps/examples/basic.rs}
```

[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`io::copy`]: https://doc.rust-lang.org/std/io/fn.copy.html
[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`Response::url`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html#method.url
[`tempfile::Builder`]: https://docs.rs/tempfile/*/tempfile/struct.Builder.html
[`tempdir()`]: https://docs.rs/tempfile/3.1.0/tempfile/struct.Builder.html#method.tempdir
