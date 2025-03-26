# Crates

The `rust_howto` book uses custom directives to insert crate links and badges.

Any left-over directives should be scrubbed by `mdbook-scrub` if the corresponding
configuration toggle is set (which is the default).

## Crate Links

Internal crate page: {{crate xyz}}
`docs.rs` link: {{docs xyz}}
Github link: {{github xyz}}
`lib.rs` link: {{lib.rs xyz}}
`crates.io` link: {{crates.io xyz}}
Website for the crate: {{web xyz}}

{{crate xyz}}
{{crate: xyz}}
{{crate xyz }}
{{crate x_y-z }}

Example of rendered links:

[`diesel`][c-diesel]⮳{{hi:diesel}}

[`mdbook-linkcheck`][c-mdbook_linkcheck-github]{{hi:mdbook-linkcheck}}⮳.

## Crate Badges

{{!crate xyz}}
{{!docs: xyz}}
{{!github xyz}}
{{!lib.rs xyz}}
{{!crates.io xyz}}
{{!web xyz}}

{{!crate: xyz}}
{{!crate: xyz }}
{{!crate: x_y-z}}

Multiple badges:

{{!crate abc def}}
{{!crate cargo-deb cargo-rpm}}

Example of rendered crate badge:

[![num][c-num-badge]][c-num]{{hi:num}}
