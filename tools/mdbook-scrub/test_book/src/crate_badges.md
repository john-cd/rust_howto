# Crate Badges

TODO: NOT IMPLEMENTED.

The `rust_howto` book uses custom directives to insert crate badges.

Any left-over directives should be scrubbed by `mdbook-scrub` if the corresponding configuration toggle is set (which is the default).

- Internal crate page: {{!crate xyz}}
  - {{! crate xyz}}
  - {{!crate xyz }}
  - {{!crate: x_y-z}}
  - {{!crate : x_y-z}}
- {{!docs xyz}}
- {{!github xyz}}
- {{!lib.rs xyz}}
- {{!crates.io xyz}}
- {{!web xyz}}

## Example of rendered crate badge

{{!docs: num}} -> [![num][c~num~docs~badge]][c~num~docs]{{hi:num}}

## Badges for Multiple Crates

- {{!crate abc def}}
- {{!crate cargo-deb cargo-rpm}}
