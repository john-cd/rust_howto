# Categories

The `rust_howto` book uses custom directives to insert category links and badges.

Any left-over directives should be scrubbed by `mdbook-scrub` if the corresponding
configuration toggle is set (which is the default).

## Category Links

{{cat: xyz}}

{{cat: xyz }}

{{ cat: xyz}}

{{cat : xyz}}

{{cat: x-y_z::a-b_c }}

With title:

{{cat: xyz | XYZ }}

Examples of rendered directives:

[Testing][cat-development-tools::testing]⮳{{hi:development-tools::testing}}

[Accessibility][cat-accessibility]⮳{{hi:accessibility}}

## Category Badges

{{catb: mathematics}}

{{catb: no-std}}

{{catb: no_std}}

{{catb: mathematics }}

{{ catb: mathematics}}

{{catb : mathematics}}

Multiple categories:

{{catb: mathematics science }}

Examples of rendered category badge blocks:

[![cat-mathematics][cat-mathematics-badge]][cat-mathematics]{{hi:Mathematics}}

[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}
