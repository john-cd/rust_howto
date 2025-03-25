# Crate directives

The `rust_howto` book uses custom directives to insert category badges.

Any left-over directives should be scrubbed by `mdbook-scrub` if the corresponding
configuration toggle is set (which is the default).

{{#crate}}

{{#crate }}

{{#crate some_crate}}

{{#crate some_crate }}

## Just the Crate Link

[`mdbook-linkcheck`][c-mdbook_linkcheck-github]{{hi:mdbook-linkcheck}}⮳.

{{c: xyz}}

{{c: xyz }}

{{c: x_y-z }}


## Crate Badge

[![num][c-num-badge]][c-num]{{hi:num}}⮳

{{cb: xyz}}

{{cb: xyz }}

{{cb: x_y-z}}
