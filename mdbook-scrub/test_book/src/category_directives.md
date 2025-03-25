# Category Directives

The `rust_howto` book uses custom directives to insert category badges.

Any left-over directives should be scrubbed by `mdbook-scrub` if the corresponding
configuration toggle is set (which is the default).

## Just the Category Link

[testing][cat-development-tools::testing]⮳{{hi:development-tools::testing}}

[Accessibility][cat-accessibility]⮳{{hi:accessibility}}

[Testing][cat-development-tools::testing]⮳{{hi:development-tools::testing}}

{{cat: xyz}}

{{cat: xyz }}

{{cat: x-y_z::a-b_c }}

{{cat: xyz | XYZ }}


## Category Badge

[![cat-mathematics][cat-mathematics-badge]][cat-mathematics]

[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

{{catbdg: mathematics}}

{{catb: mathematics }}

{{catb: mathematics science }}

{{catb: no-std }}

{{catb: no_std }}
