# Crate Links

TODO: NOT IMPLEMENTED.

The `rust_howto` book uses custom directives to insert crate links and badges.

Any left-over directives should be scrubbed by `mdbook-scrub` if the corresponding configuration toggle is set (which is the default).

- Internal crate page: {{crate xyz}}
- Variations:
  - {{ crate xyz}}
  - {{crate xyz }}
  - {{crate: xyz}}
  - {{crate : xyz}}
  - {{crate x_y-z}}
- `docs.rs` link: {{docs xyz}}
- Github link: {{github xyz}}
- `lib.rs` link: {{lib.rs xyz}}
- `crates.io` link: {{crates.io xyz}}
- Website for the crate: {{web xyz}}

## Example of rendered links

- {{docs diesel}} -> [`diesel`][c~diesel~docs]↗{{hi:diesel}}
- {{github mdbook-linkcheck}} -> [`mdbook-linkcheck` (GitHub)][c~mdbook-linkcheck~repo]↗{{hi:mdbook-linkcheck}}.
