# Category Links

TODO: NOT IMPLEMENTED.

The `rust_howto` book uses custom directives to insert category links.

Any left-over directives should be scrubbed by `mdbook-scrub` if the corresponding configuration toggle is set (which is the default).

- {{cat xyz}}
- {{cat xyz }}
- {{ cat xyz}}
- {{cat: xyz}}
- {{cat : xyz}}
- {{cat x-y_z::a-b_c }}

{{cat: accessibility}} -> [Accessibility][cat~accessibility]↗{{hi:accessibility}}

## Category Links with Title

- {{cat xyz | XYZ }}
- {{cat development-tools::testing | Testing }} -> [Testing][cat~development-tools::testing]↗{{hi:development-tools::testing}}
