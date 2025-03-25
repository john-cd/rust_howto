# Crates

The `rust_howto` book uses custom directives to insert crate links and badges.

Any left-over directives should be scrubbed by `mdbook-scrub` if the corresponding
configuration toggle is set (which is the default).

## Crate Links

{{docs: xyz}}
{{github: xyz}}
{{lib.rs: xyz}}
{{crate: xyz}}

{{crate: xyz}}
{{crate: xyz }}
{{crate: x_y-z }}

Example of rendered crate link:

[`mdbook-linkcheck`][c-mdbook_linkcheck-github]{{hi:mdbook-linkcheck}}⮳.

## Crate Badges

{{crateb}}

{{crateb: }}

{{crateb: xyz}}

{{crateb: xyz }}

{{crateb: x_y-z}}

Multiple badges:

{{crateb: abc def}}

Example of rendered crate badge:

[![num][c-num-badge]][c-num]{{hi:num}}⮳

##
