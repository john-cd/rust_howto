# Crate Blocks

The `rust_howto` book uses custom directives to insert crate blocks.

Any left-over directives should be scrubbed by `mdbook-scrub` if the corresponding
configuration toggle is set (which is the default).

{{#crate: crt}}

{{#crate: crt }}

{{#crate: x_y-z}}

With additional categories:

{{#crate: crt cat1 cat-2 cat-2-2 cat3::sub-cat-3 }}
