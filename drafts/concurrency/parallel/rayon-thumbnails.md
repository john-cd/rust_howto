## Generate jpg thumbnails in parallel

[![rayon-badge]][rayon] [![glob-badge]][glob] [![image-badge]][image] [![cat-concurrency-badge]][cat-concurrency] [![cat-filesystem-badge]][cat-filesystem]

This example generates thumbnails for all .jpg files in the current directory
then saves them in a new folder called `thumbnails`.

[`glob::glob_with`] finds jpeg files in current directory. `rayon` resizes
images in parallel using [`par_iter`] calling  [`DynamicImage::resize`].

```rust,editable,no_run
{#include ../../../deps/examples/rayon-thumbnails.rs}
```

[`glob::glob_with`]: https://docs.rs/glob/*/glob/fn.glob_with.html
[`par_iter`]: https://docs.rs/rayon/*/rayon/iter/trait.IntoParallelRefIterator.html#tymethod.par_iter
[`DynamicImage::resize`]: https://docs.rs/image/*/image/enum.DynamicImage.html#method.resize
